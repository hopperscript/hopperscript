mod define;
mod types;
mod typess;

extern crate pest;
#[macro_use]
extern crate pest_derive;

/// Main module for the compiler
pub mod compiler {
    use pest::{Parser, iterators::Pair};
    use uuid::Uuid;

    use crate::types::{Project, Variable};

    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    #[derive(Parser)]
    #[grammar = "compiler/script.pest"]
    struct Script;

    pub fn compile(i: &str) -> Project {
        let mut proj = Project {
            variables: vec![]
        };

        let p: Pair<'_, Rule> = Script::parse(Rule::file, i).expect("a").next().unwrap();

        for v in p.into_inner() {
            match v.as_rule() {
                Rule::define => {
                    let mut i = v.into_inner();

                    let typ = i.next().unwrap().as_str();
                    
                    match typ {
                        "var" => {
                            let name = i.next().unwrap().as_str();
                            proj.variables.push(Variable {
                                name: name.to_string(),
                                typ: 8003,
                                object_id_string: giv_me_uuid()
                            })
                        },
                        _ => {}
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
