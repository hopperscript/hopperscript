mod define;
mod types;
mod typess;

extern crate pest;
#[macro_use]
extern crate pest_derive;

/// Main module for the compiler
pub mod compiler {
    use pest::{Parser, iterators::Pair};

    use crate::types::Project;

    #[derive(Parser)]
    #[grammar = "compiler/script.pest"]
    struct Script;

    pub fn compile(i: &str) -> String {
        let mut proj = Project {
            variables: vec![]
        };

        let p: Pair<'_, Rule> = Script::parse(Rule::file, i).expect("a").next().unwrap();

        for v in p.into_inner() {
            match v.as_rule() {
                Rule::define => {
                    let mut i = v.into_inner();

                    let typ = i.next().unwrap().as_str();
                }
                _ => {}
            }
        }

        "a".to_string()
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
