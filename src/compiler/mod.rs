mod export;
mod getdata;
mod types;

/// Main module for the compiler
pub mod compiler {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;
    use chumsky::text::ident;
    use rhai::serde::{from_dynamic, to_dynamic};
    use rhai::{Dynamic, Map};
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    pub use crate::export::to_json;
    use crate::getdata::{self, CompiledData};
    use crate::types::{
        Ability, Block, ControlScript, Datum, EventParam, Param, Project, Rule, Scene, Variable,
    };

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string().to_uppercase()
    }

    /// turn `Vec<Values>` to `Vec<String>` to `Dynamic`
    fn transform_vals(params: Vec<Values>, proj: &Project) -> Dynamic {
        to_dynamic::<Vec<Value>>(
            params
                .into_iter()
                .map(|v| match v {
                    Values::Object(v) => Value {
                        value: proj
                            .objects
                            .to_owned()
                            .into_iter()
                            .find(|i| i.name == v)
                            .expect("Object not found")
                            .id,
                        datum: None,
                    },

                    Values::Str(v) => Value {
                        value: v,
                        datum: None,
                    },

                    Values::Variable(v) => {
                        let var = proj
                            .variables
                            .to_owned()
                            .into_iter()
                            .find(|p| p.name == v)
                            .unwrap()
                            .object_id_string;

                        Value {
                            value: "".to_string(),
                            datum: Some(
                                to_dynamic(Datum {
                                    variable: Some(var),
                                    typ: 8003,
                                    block_class: None,
                                    params: None,
                                })
                                .unwrap(),
                            ),
                        }
                    }
                })
                .collect(),
        )
        .unwrap()
    }

    pub type Span = std::ops::Range<usize>;

    #[derive(Clone, Debug)]
    pub enum Values {
        Object(String),
        Str(String),
        Variable(String),
    }

    #[derive(Clone, Debug)]
    pub enum DefineTypes {
        Object(String),
        Variable,
        Ability(Option<Vec<BlockAST>>),
    }

    #[derive(Clone, Debug)]
    pub enum Script {
        Define {
            typ: DefineTypes,
            name: String,
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

    #[derive(Debug, Clone, PartialEq)]
    pub enum AstTypes {
        Block,
        Ability,
    }

    #[derive(Debug, Clone)]
    pub struct BlockAST {
        pub name: String,
        pub params: Vec<Values>,
        pub typ: AstTypes,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Value {
        pub value: String,
        pub datum: Option<Dynamic>,
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

        // println!("{:#?}", a);

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

        let obj_ref = just('o').ignore_then(stri).map(Values::Object);
        let var_ref = just('v').ignore_then(stri).map(Values::Variable);

        let value = stri.map(Values::Str).or(obj_ref).or(var_ref);

        let abil = just("ability!")
            .ignore_then(stri.delimited_by(just('('), just(')')))
            .padded()
            .map(|v| BlockAST {
                name: v,
                params: vec![],
                typ: AstTypes::Ability,
            });

        let block = ident()
            .then(
                value
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .padded()
            .map(|(a, b)| BlockAST {
                name: a,
                params: b,
                typ: AstTypes::Block,
            });

        let block_or_abil = block.or(abil);

        let var = just("var")
            .padded()
            .ignore_then(stri.padded())
            .map(|b| Script::Define {
                typ: DefineTypes::Variable,
                name: b,
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
                typ: DefineTypes::Object(c),
                name: a,
            });

        let ability_def = just("ability")
            .padded()
            .ignore_then(stri.padded())
            .then(
                block_or_abil
                    .repeated()
                    .or_not()
                    .delimited_by(just('{'), just('}'))
                    .padded(),
            )
            .map(|(a, c)| Script::Define {
                typ: DefineTypes::Ability(c),
                name: a,
            });

        let def = just("define").ignore_then(var.or(obj.or(ability_def)));

        let rule = just("when")
            .ignore_then(ident().padded())
            .then(
                obj_ref
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .then(
                block_or_abil
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
                .as_millis()
                / 1000
                * 65536,
            36,
        )
        .to_string();

        let mut proj = Project {
            variables: vec![],
            uuid,
            objects: vec![],
            rules: vec![],
            abilities: vec![],
            scenes: vec![Scene {
                name: "My Scene".to_string(),
                objects: vec![],
            }],
            event_params: vec![],
        };

        for v in p.to_owned() {
            match v {
                Script::Define { typ, name } => {
                    match typ {
                        DefineTypes::Variable => proj.variables.push(Variable {
                            name: name.to_string(),
                            typ: 8003,
                            object_id_string: giv_me_uuid(),
                        }),

                        DefineTypes::Object(val) => {
                            // TODO: use ariadne

                            let f = bd
                                .obj
                                .to_owned()
                                .into_iter()
                                .find(|v| v.fn_name() == val)
                                .expect("Object not found");

                            let res = f
                                .call(&bd.eng, &bd.ast, (name,))
                                .expect("Failed to get object");

                            let mut act_res: Map =
                                from_dynamic(&res).expect("Failed to get object");

                            act_res.insert("rules".into(), (vec![] as Vec<String>).into());

                            // get id from res when needed

                            proj.scenes[0].objects.push(
                                act_res
                                    .get("objectID")
                                    .expect("Failed to insert object to scene")
                                    .to_string(),
                            );
                            proj.event_params.push(EventParam {
                                description: act_res
                                    .get("name")
                                    .expect("Failed to add object")
                                    .to_string(),
                                block_type: 8000,
                                id: giv_me_uuid(),
                                object_id: Some(
                                    act_res
                                        .get("objectID")
                                        .expect("Failed to add object")
                                        .to_string(),
                                ),
                            });
                            proj.objects
                                .push(from_dynamic(&act_res.into()).expect("Failed to get object"))
                        }

                        DefineTypes::Ability(mut blocks) => {
                            let id = giv_me_uuid();

                            let mut ability_json = Ability {
                                ability_id: id,
                                blocks: vec![],
                                created_at: 0,
                                name: Some(name),
                            };

                            for c in blocks.get_or_insert(vec![]) {
                                let ptr = bd
                                    .blocks
                                    .to_owned()
                                    .into_iter()
                                    .find(|v| v.fn_name() == c.name)
                                    .expect("Block not found");

                                let transformed = transform_vals(c.params.to_owned(), &proj);

                                let call = ptr
                                    .call(&bd.eng, &bd.ast, (transformed,))
                                    .expect("Failed to get block");

                                ability_json.blocks.push(
                                    from_dynamic::<Block>(&call).expect("Failed to get block"),
                                );
                            }

                            proj.abilities.push(ability_json)
                        } //_ => todo!(),
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

                                let transformed = transform_vals(params, &proj);

                                let res = f
                                    .call(
                                        &bd.eng,
                                        &bd.ast,
                                        (
                                            object.to_owned().id,
                                            transformed,
                                            to_dynamic(&proj).unwrap(),
                                        ),
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
                                    name: None,
                                };

                                for c in con {
                                    if c.typ == AstTypes::Block {
                                        let ptr = bd
                                            .blocks
                                            .to_owned()
                                            .into_iter()
                                            .find(|v| v.fn_name() == c.name)
                                            .expect("Block not found");

                                        let transformed = transform_vals(c.params, &proj);

                                        let call = ptr
                                            .call(&bd.eng, &bd.ast, (transformed,))
                                            .expect("Failed to get block");

                                        ability_json.blocks.push(
                                            from_dynamic::<Block>(&call)
                                                .expect("Failed to get block"),
                                        );
                                    } else {
                                        let ability = proj
                                            .abilities
                                            .clone()
                                            .into_iter()
                                            .find(|v| {
                                                v.name.as_ref().expect("Rule not found?") == &c.name
                                            })
                                            .expect("Rule not found")
                                            .ability_id;

                                        ability_json.blocks.push(Block {
                                            typ: 123,
                                            description: c.name,
                                            control_script: Some(ControlScript {
                                                ability_id: ability.to_owned(),
                                            }),
                                            block_class: "control".to_string(),
                                            parameters: None,
                                        });
                                    }
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
