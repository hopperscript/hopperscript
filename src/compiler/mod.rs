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
    use std::vec;
    use uuid::Uuid;

    pub use crate::export::to_json;
    use crate::getdata::{self, CompiledData};
    use crate::types::{
        Ability, Block, Datum, EventParam, Object, Param, Project, Rule, Scene, Variable,
    };

    extern crate regex;

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
        pub params: Vec<Values>,
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

        gen_project(&a.unwrap(), getdata::generate_data("src/compiler/std.yaml"))
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
        let var_ref = just('v').ignore_then(stri).map(Values::Variable);

        let def = just("define").ignore_then(var.or(obj));

        let value = stri.map(Values::Str).or(obj_ref).or(var_ref);

        let block = ident()
            .then(
                value
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .padded()
            .map(|(a, b)| BlockAST { name: a, params: b });

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
                                .objects
                                .get(val.as_ref().expect("What object?"))
                                .expect("Object not found(?)");

                            let id = giv_me_uuid();

                            let obj = Object {
                                typ: f.typ,
                                filename: f.filename.to_owned(),
                                name: name.clone(),
                                id: id.clone(),
                                rules: vec![],
                                x: 1,
                                y: 1,
                            };

                            proj.scenes[0].objects.push(id.clone());
                            proj.event_params.push(EventParam {
                                description: name,
                                block_type: 8000,
                                id: giv_me_uuid(),
                                object_id: Some(id),
                            });
                            proj.objects.push(obj)
                        }

                        _ => todo!(),
                    }
                }

                Script::On { obj, con } => {
                    let ob = proj
                        .objects
                        .iter()
                        .position(|p| p.name == obj)
                        .expect("No object with that name");

                    let object = proj.objects[ob].to_owned();

                    for v in con {
                        match v {
                            Script::Rule { name, con, params } => {
                                let rule = bd.rules.get(&name).expect("Rule not found.");

                                let ability = giv_me_uuid();

                                let id = giv_me_uuid();

                                let r = Rule {
                                    rule_block_type: 6000,
                                    object_id: object.to_owned().id,
                                    id: id.clone(),
                                    ability_id: ability.to_owned(),
                                    params: vec![Param {
                                        datum: rule.datum.to_owned().or(Some(Datum {
                                            typ: rule.typ.expect("For some reason, there's a problem with the block data."),
                                            params: None,
                                            block_class: Some("operator".to_string()),
                                            variable: None,
                                        })),
                                        value: "".to_string(),
                                        typ: 52,
                                        key: "".to_string(),
                                        default_value: "".to_string(),
                                        variable: None,
                                    }],
                                };

                                let mut ability_json = Ability {
                                    ability_id: ability,
                                    blocks: vec![],
                                    created_at: 0,
                                };

                                for c in con {
                                    let ptr = bd.blocks.get(&c.name).expect("Block not found.");

                                    let block = Block {
                                        block_class: "method".to_string(),
                                        typ: ptr.typ,
                                        description: ptr.description.to_owned(),
                                        parameters: None,
                                    };

                                    ability_json.blocks.push(block);
                                }

                                proj.abilities.push(ability_json);

                                proj.objects[ob].rules.push(id);

                                proj.rules.push(r)
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
