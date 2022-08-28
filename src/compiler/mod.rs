mod getdata;
mod types;

/// Main module for the compiler
pub mod compiler {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;
    use chumsky::text::ident;
    use rhai::serde::from_dynamic;
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    use crate::getdata::{self, CompiledData};
    use crate::types::{Project, Rule, Variable};

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    pub type Span = std::ops::Range<usize>;

    #[derive(Clone, Debug)]
    pub enum Script {
        Define {
            typ: String,
            name: String,
            val: Option<String>,
        },
        Str(String),
        Loop(Vec<Self>),
        On {
            obj: String,
            con: Vec<Script>, //probably temporary
        },
        Rule {
            name: String,
            con: String, //temp
        },
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

        let def = just("define").ignore_then(var.or(obj));

        let rule = just("when")
            .ignore_then(ident().padded())
            .then(ident().or_not().delimited_by(just('{'), just('}')).padded())
            .map(|(a, mut b)| Script::Rule { name: a, con: b.get_or_insert("".to_string()).to_string() });

        let on = just("for")
            .ignore_then(stri.padded())
            .then(
                def.or(rule)
                    .padded()
                    .repeated()
                    .delimited_by(just('{'), just('}'))
                    .padded().or_not(),
            )
            .map(|(a, mut b)| Script::On { obj: a, con: b.get_or_insert(vec![]).to_vec() });

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
        };

        for v in p {
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

                            let res = f.call(&bd.eng, &bd.ast, ()).expect("Failed to get object");

                            // get id from res when needed

                            proj.objects
                                .push(from_dynamic(&res).expect("Failed to get object"))
                        }

                        _ => todo!(),
                    }
                }

                Script::On { obj: _, con } => {
                    for v in con {
                        match v {
                            Script::Rule { name: _, con: _ } => proj.rules.push(Rule {
                                rule_block_type: 6000,
                                object_id: "".to_string(),
                                id: "".to_string(),
                                ability_id: "".to_string(),
                                parameters: vec![],
                            }),

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
