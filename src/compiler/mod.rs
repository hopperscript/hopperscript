mod export;
mod getdata;
mod types;

// gosh this is such a mess
/// Main module for the compiler
pub mod compiler {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
    use chumsky::prelude::*;
    use chumsky::text::ident;
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    pub use crate::export::to_json;
    use crate::getdata::{self, CompiledData};
    use crate::types::{
        Ability, Block, ControlScript, Datum, EventParam, Object, Param, Project, Rule, Scene,
        Variable,
    };

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string().to_uppercase()
    }

    fn is_hex(c: char) -> bool {
        "0123456789abcdefABCDEF".contains(c)
    }

    fn value_to_param(value: &Value, typ: i32, variable: Option<String>) -> Param {
        Param {
            datum: value.datum.to_owned(),
            default_value: "".to_string(),
            key: "".to_string(),
            typ,
            value: value.value.to_owned(),
            variable,
        }
    }

    /// turn `Vec<Values>` to `Vec<String>` to `Dynamic`
    fn transform_vals(params: Vec<Values>, proj: &Project) -> Vec<Value> {
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

                Values::Variable(v, code) => {
                    let var = proj
                        .variables
                        .to_owned()
                        .into_iter()
                        .find(|p| p.name == v)
                        .expect("Variable does not exist?")
                        .object_id_string;

                    Value {
                        value: "".to_string(),
                        datum: Some(Datum {
                            variable: Some(var),
                            typ: 8000 + code,
                            block_class: None,
                            params: None,
                            object: None,
                        }),
                    }
                }

                Values::ObjectVariable(o, v) => {
                    //copy-paste
                    let var = proj
                        .variables
                        .to_owned()
                        .into_iter()
                        .find(|p| p.name == v)
                        .unwrap()
                        .object_id_string;
                    let obj = proj
                        .objects
                        .to_owned()
                        .into_iter()
                        .find(|p| p.name == o)
                        .unwrap()
                        .id;

                    Value {
                        value: "".to_string(),
                        datum: Some(Datum {
                            variable: Some(var),
                            typ: 8000,
                            block_class: None,
                            params: None,
                            object: Some(obj),
                        }),
                    }
                }

                Values::Conditional(v1, cond, v2) => {
                    let id = match cond.as_str() {
                        "==" => 1000,
                        "!=" => 1001,
                        "<" => 1002,
                        ">" => 1003,
                        "and" => 1004,
                        "or" => 1005,
                        ">=" => 1006,
                        "<=" => 1007,
                        &_ => 0,
                    };

                    Value {
                        value: "".to_string(),
                        datum: Some(Datum {
                            block_class: Some("conditionalOperator".to_string()),
                            object: None,
                            typ: id,
                            variable: None,
                            params: Some(vec![
                                value_to_param(&transform_vals(vec![*v1], proj)[0], 42, None),
                                value_to_param(&transform_vals(vec![*v2], proj)[0], 42, None),
                            ]),
                        }),
                    }
                }
            })
            .collect()
    }

    pub type Span = std::ops::Range<usize>;

    #[derive(Clone, Debug)]
    pub enum Values {
        Object(String),
        Str(String),
        Variable(String, i32),
        ObjectVariable(String, String),
        Conditional(Box<Values>, String, Box<Values>),
    }

    #[derive(Clone, Debug)]
    pub enum DefineTypes {
        Object(String),
        /// i32 = the "code"
        Variable(i32),
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

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Value {
        pub value: String,
        pub datum: Option<Datum>,
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
            getdata::generate_blocks(),
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

        let hex_color_short = just::<char, char, Simple<char>>('#')
            .chain(filter(|c: &char| is_hex(*c)).repeated().exactly(3))
            .collect::<String>();
        let hex_color_long = just::<char, char, Simple<char>>('#')
            .chain(filter(|c: &char| is_hex(*c)).repeated().exactly(6))
            .collect::<String>();

        let hex_color = hex_color_long.or(hex_color_short);

        let number = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
            .repeated()
            .at_least(1)
            .collect::<String>();

        let abil = just("ability!")
            .ignore_then(stri.delimited_by(just('('), just(')')))
            .padded()
            .map(|v| BlockAST {
                name: v,
                params: vec![],
                typ: AstTypes::Ability,
            });

        let var = just("var")
            .padded()
            .ignore_then(stri.padded())
            .map(|b| Script::Define {
                typ: DefineTypes::Variable(3),
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

        let obj_ref = just('o').ignore_then(stri).map(Values::Object);
        let var_ref = just('v').ignore_then(stri).map(|a| Values::Variable(a, 3));
        let objvar_ref = just('v')
            .ignore_then(stri)
            .then_ignore(just('.'))
            .then(stri)
            .map(|(obj, var)| Values::ObjectVariable(obj, var));
        let selfvar_ref = just("v Self.")
            .ignore_then(stri)
            .map(|a| Values::Variable(a, 4));
        let value = stri
            .map(Values::Str)
            .or(hex_color.map(Values::Str))
            .or(number.map(Values::Str))
            .or(obj_ref)
            .or(objvar_ref)
            .or(var_ref)
            .or(selfvar_ref);

        let block = ident()
            .then(
                value
                    .padded()
                    .separated_by(just(','))
                    .delimited_by(just('('), just(')')),
            )
            .padded()
            .map(|(a, b)| BlockAST {
                name: a,
                params: b,
                typ: AstTypes::Block,
            });

        let block_or_abil = block.or(abil);

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

        let conditional = value
            .then(just("==").or(just("!=")).padded())
            .then(value)
            .map(|((a, b), c)| Values::Conditional(Box::new(a), b.to_string(), Box::new(c)));
        let rule = just("when")
            .ignore_then(just("cond").map(|s| s.to_string()).or(ident()).padded())
            .then(
                obj_ref
                    .or(conditional)
                    .padded()
                    .separated_by(just(','))
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
            })
            .padded();

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
            fn convert_to_block(
                c: &BlockAST,
                bd: &CompiledData,
                ability_json: &mut Ability,
                proj: &Project,
            ) {
                if c.typ == AstTypes::Block {
                    let ptr = bd
                        .blocks
                        .to_owned()
                        .into_iter()
                        .find(|v| v.name == c.name)
                        .expect("Block not found");

                    let transformed = transform_vals(c.params.clone(), &proj);
                    let params = transformed
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| Param {
                            datum: v.datum,
                            default_value: "".to_string(),
                            key: "".to_string(),
                            typ: match ptr.parameters[i].typ.as_str() {
                                "num" => 57,
                                "evt" => 50,
                                &_ => 0,
                            },
                            value: v.value,
                            variable: None,
                        })
                        .collect::<Vec<Param>>();

                    let block = Block {
                        block_class: "method".to_string(),
                        typ: ptr.id,
                        description: ptr.label,
                        parameters: Some(params),
                        control_script: None,
                    };

                    ability_json.blocks.push(block);
                } else {
                    let ability = if &c.name == ability_json.name.get_or_insert("".to_string()) {
                        ability_json.ability_id.clone()
                    } else {
                        proj.abilities
                            .clone()
                            .into_iter()
                            .find(|v| v.name.as_ref().expect("Rule not found?") == &c.name)
                            .expect("Ability not found")
                            .ability_id
                    };

                    ability_json.blocks.push(Block {
                        typ: 123,
                        description: c.name.clone(),
                        control_script: Some(ControlScript {
                            ability_id: ability,
                        }),
                        block_class: "control".to_string(),
                        parameters: None,
                    });
                }
            }

            match v {
                Script::Define { typ, name } => {
                    match typ {
                        DefineTypes::Variable(code) => proj.variables.push(Variable {
                            name: name.to_string(),
                            typ: 8000 + code,
                            object_id_string: giv_me_uuid(),
                        }),

                        DefineTypes::Object(val) => {
                            // TODO: use ariadne

                            let f = bd
                                .obj
                                .to_owned()
                                .into_iter()
                                .find(|v| v.name == val)
                                .expect("Object not found");

                            let act_res = Object {
                                filename: "".to_string(),
                                typ: f.id,
                                name,
                                id: giv_me_uuid(),
                                rules: vec![] as Vec<String>,
                                x: 10, //dont forget!!
                                y: 10,
                            };

                            let actbor = &act_res;

                            proj.scenes[0].objects.push(actbor.id.to_owned());
                            proj.event_params.push(EventParam {
                                description: actbor.name.to_owned(),
                                block_type: 8000,
                                id: giv_me_uuid(),
                                object_id: Some(actbor.id.to_owned()),
                            });
                            proj.objects.push(act_res)
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
                                convert_to_block(&c, &bd, &mut ability_json, &proj);
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

                                let transformed = transform_vals(params, &proj);

                                //make this a func for reuse with the block part
                                let datum = if name != "cond" {
                                    let f = bd
                                        .rules
                                        .to_owned()
                                        .into_iter()
                                        .find(|v| v.name == name)
                                        .expect("Rule not found");

                                    let paramets = transformed
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, v)| Param {
                                            datum: v.datum,
                                            default_value: "".to_string(),
                                            key: "".to_string(),
                                            typ: match f.parameters[i].typ.as_str() {
                                                "num" => 57,
                                                "evt" => 50,
                                                &_ => 0,
                                            },
                                            value: v.value.to_owned(),
                                            variable: if f.parameters[i].typ.as_str() == "evt" {
                                                Some(
                                                    proj.event_params
                                                        .to_owned()
                                                        .into_iter()
                                                        .find(|ev| {
                                                            ev.object_id.as_ref().unwrap()
                                                                == &v.value
                                                        })
                                                        .expect("Object not found")
                                                        .id,
                                                )
                                            } else {
                                                None
                                            },
                                        })
                                        .collect::<Vec<Param>>();

                                    Datum {
                                        block_class: Some("operator".to_string()),
                                        typ: f.id,
                                        object: None,
                                        variable: None,
                                        params: Some(paramets),
                                    }
                                } else {
                                    value_to_param(&transformed[0], 52, None).datum.unwrap()
                                };

                                // let res = f
                                //     .call(
                                //         &bd.eng,
                                //         &bd.ast,
                                //         (
                                //             object.to_owned().id,
                                //             transformed,
                                //             to_dynamic(&proj).unwrap(),
                                //         ),
                                //     )
                                //     .expect("Failed to get rule");

                                let ability = giv_me_uuid();

                                let rule = Rule {
                                    rule_block_type: 6000,
                                    object_id: object.id,
                                    id: giv_me_uuid(),
                                    ability_id: ability.to_owned(),
                                    params: vec![Param {
                                        default_value: "".to_string(),
                                        key: "".to_string(),
                                        datum: Some(datum),
                                        typ: 52,
                                        value: "".to_string(),
                                        variable: None,
                                    }],
                                };

                                let mut ability_json = Ability {
                                    ability_id: ability,
                                    blocks: vec![],
                                    created_at: 0,
                                    name: None,
                                };

                                for c in con {
                                    convert_to_block(&c, &bd, &mut ability_json, &proj);
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
