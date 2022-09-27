mod getdata;
mod types;

/// Main module for the compiler
pub mod compiler {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;
    use chumsky::text::ident;
    use rhai::serde::{from_dynamic, to_dynamic};
    use rhai::Map;
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    use crate::getdata::{self, CompiledData};
    use crate::types::{Ability, Block, Param, Project, Rule, Variable};

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    pub type Span = std::ops::Range<usize>;

    #[derive(Clone, Debug)]
    pub enum Values {
        Object(String),
    }

    #[derive(Clone, Debug)]
    pub enum Script {
        Define {
            typ: String,
            name: String,
            val: Option<String>,
        },
        Loop(Vec<Self>),
        On {
            obj: String,
            con: Vec<Script>,
        },
        Rule {
            name: String,
            con: Vec<BlockAST>,
            params: Vec<Values>,
        },
    }

    #[derive(Debug, Clone)]
    pub struct BlockAST {
        pub name: String,
    }

    /// The main compile fn
    ///
    /// Just throw a string that needs to be compiled
    ///
    /// I mean a `str`
    pub fn compile(s: &str) -> Project {
        let (a, errs) = ast().parse_recovery(s);

        // Report::build(ReportKind::Error, (), 0)
        //.with_message("No such block")
        //.with_label(Label::new(ln..ln+name.len())
        //.with_message(format!("Block \"{}\" not found", name).fg(Color::Red))
        //.with_color(Color::Red))
        //.finish()
        //.print(Source::from(format!("{}{}","\n".repeat(ln),line))).unwrap();panic!();

        // very much copied code
        // also very experimental
        errs.into_iter().for_each(|v| {
            let msg = if let chumsky::error::SimpleReason::Custom(msg) = v.reason() {
                msg.clone()
            } else {
                format!(
                    "{}{}, expected {}",
                    if v.found().is_some() {
                        "Unexpected token"
                    } else {
                        "Unexpected end of input"
                    },
                    if let Some(label) = v.label() {
                        format!(" while parsing {}", label)
                    } else {
                        String::new()
                    },
                    if v.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        v.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                )
            };

            Report::build(ReportKind::Error, (), v.span().start)
                .with_message(msg)
                .with_label(
                    Label::new(v.span())
                        .with_message(match v.reason() {
                            chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                            _ => format!(
                                "Unexpected {}",
                                v.found()
                                    .map(|c| format!("token {}", c.fg(Color::Red)))
                                    .unwrap_or_else(|| "end of input".to_string())
                            ),
                        })
                        .with_color(Color::Red),
                )
                .finish()
                .print(Source::from(&s))
                .unwrap();

            // panic?
        });

        println!("{:#?}", a);

        gen_project(
            &a.unwrap(),
            getdata::generate_data("src/compiler/data.rhai"),
        )
    }

    /// Generate the "AST" or whatever
    fn ast() -> impl Parser<char, Vec<Script>, Error = Simple<char>> {
        let escape = just('\\').ignore_then(
            just('\\')
                .or(just('/'))
                .or(just('"'))
                .or(just('n').to('\n'))
                .or(just('t').to('\t')),
        );
        let stri = just('"')
            .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
            .then_ignore(just('"'))
            .collect::<String>();

        let var = just("var")
            .padded()
            .then(stri.padded())
            .map(|(a, b)| Script::Define {
                typ: a.to_string(),
                name: b,
                val: None,
            });

        let obj_ty = just::<_, _, Simple<char>>("objects")
            .ignore_then(just('.'))
            .ignore_then(ident());

        let obj = just("object")
            .padded()
            .ignore_then(stri.padded())
            .then_ignore(just('=').padded())
            .then(obj_ty.padded())
            .map(|(a, c)| Script::Define {
                typ: "obj".to_string(),
                name: a,
                val: Some(c),
            });

        let obj_ref = just('o').ignore_then(stri).map(Values::Object);

        let def = just("define").ignore_then(var.or(obj));

        let block = ident()
            .then_ignore(just("()"))
            .padded()
            .map(|a| BlockAST { name: a });

        let rule = just("when")
            .ignore_then(ident().padded())
            .then(
                obj_ref
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .then(
                block
                    .repeated()
                    .or_not()
                    .delimited_by(just('{'), just('}'))
                    .padded(),
            )
            .map(|((a, c), mut b)| Script::Rule {
                name: a,
                con: b.get_or_insert(vec![]).to_vec(),
                params: c,
            });

        let on = just("for")
            .ignore_then(stri.padded())
            .then(
                def.or(rule)
                    .padded()
                    .repeated()
                    .delimited_by(just('{'), just('}'))
                    .padded()
                    .or_not(),
            )
            .map(|(a, mut b)| Script::On {
                obj: a,
                con: b.get_or_insert(vec![]).to_vec(),
            });

        def.or(on)
            .recover_with(skip_then_retry_until([]))
            .padded()
            .repeated()
    }

    /// Generate the project
    fn gen_project(p: &Vec<Script>, bd: CompiledData) -> Project {
        use radix_fmt::radix;

        let uuid = radix(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Can't generate UUID for some reason")
                .as_millis(),
            32,
        )
        .to_string();

        let mut proj = Project {
            variables: vec![],
            uuid,
            objects: vec![],
            rules: vec![],
            abilities: vec![],
        };

        for v in p.to_owned() {
            match v {
                Script::Define { typ, name, val } => {
                    match typ.as_str() {
                        "var" => proj.variables.push(Variable {
                            name: name.to_string(),
                            typ: 8003,
                            object_id_string: giv_me_uuid(),
                        }),

                        "obj" => {
                            // TODO: use ariadne

                            let f = bd
                                .obj
                                .to_owned()
                                .into_iter()
                                .find(|v| v.fn_name() == val.as_ref().expect("What object?"))
                                .expect("Object not found");

                            let res = f
                                .call(&bd.eng, &bd.ast, (name,))
                                .expect("Failed to get object");

                            let mut act_res: Map =
                                from_dynamic(&res).expect("Failed to get object");

                            act_res.insert("rules".into(), (vec![] as Vec<String>).into());

                            // get id from res when needed

                            proj.objects
                                .push(from_dynamic(&act_res.into()).expect("Failed to get object"))
                        }

                        _ => todo!(),
                    }
                }

                Script::On { obj, con } => {
                    for v in con {
                        match v {
                            Script::Rule { name, con, params } => {
                                let ob = proj
                                    .objects
                                    .iter()
                                    .position(|p| p.name == obj)
                                    .expect("No object with that name");
                                let object = proj.objects[ob].to_owned();

                                let f = bd
                                    .rules
                                    .to_owned()
                                    .into_iter()
                                    .find(|v| v.fn_name() == name)
                                    .expect("Rule not found");

                                let transformed: Vec<String> = params
                                    .into_iter()
                                    .map(|v| match v {
                                        Values::Object(v) => {
                                            proj.objects
                                                .to_owned()
                                                .into_iter()
                                                .find(|i| i.name == v)
                                                .expect("Object not found")
                                                .id
                                        }
                                    })
                                    .collect();

                                let res = f
                                    .call(
                                        &bd.eng,
                                        &bd.ast,
                                        (object.to_owned().id, to_dynamic(transformed).unwrap()),
                                    )
                                    .expect("Failed to get rule");

                                let act_res =
                                    from_dynamic::<Vec<Param>>(&res).expect("Failed to get rule");

                                let ability = giv_me_uuid();

                                let rule = Rule {
                                    rule_block_type: 6000,
                                    object_id: object.id,
                                    id: giv_me_uuid(),
                                    ability_id: ability.to_owned(),
                                    params: act_res,
                                };

                                let mut ability_json = Ability {
                                    ability_id: ability,
                                    blocks: vec![],
                                    created_at: 0,
                                };

                                for c in con {
                                    let ptr = bd
                                        .blocks
                                        .to_owned()
                                        .into_iter()
                                        .find(|v| v.fn_name() == c.name)
                                        .expect("Block not found");

                                    let call = ptr
                                        .call(&bd.eng, &bd.ast, ())
                                        .expect("Failed to get block");

                                    ability_json.blocks.push(
                                        from_dynamic::<Block>(&call).expect("Failed to get block"),
                                    );
                                }

                                proj.abilities.push(ability_json);

                                proj.objects[ob].rules.push(rule.to_owned().id);

                                proj.rules.push(rule)
                            }

                            _ => {}
                        }
                    }
                }

                _ => {}
            }
        }

        proj
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
