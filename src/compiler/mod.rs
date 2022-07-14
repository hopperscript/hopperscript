mod types;

/// Main module for the compiler
pub mod compiler {
    use chumsky::error::Cheap;
    use chumsky::prelude::*;
    use uuid::Uuid;

    use crate::types::{Project, Variable};

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
    }

    /// The main compile fn
    ///
    /// Just throw a string that needs to be compiled
    ///
    /// I mean a `str`
    pub fn compile(s: &str) -> Project {
        let a = ast().parse(s);
        gen_project(&a.unwrap())
    }

    /// Generate the "AST" or whatever
    fn ast() -> impl Parser<char, Vec<Script>, Error = Cheap<char>> {
        let stri = just::<_, _, Cheap<char>>('"')
            .ignore_then(filter(|c| *c != '"').repeated())
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

        let obj = just("object")
            .padded()
            .ignore_then(stri.padded())
            .then_ignore(just('=').padded())
            .then(stri.padded())
            .map(|(a, c)| Script::Define {
                typ: "obj".to_string(),
                name: a,
                val: Some(c),
            });

        let def = just("define").ignore_then(var.or(obj));

        def.recover_with(skip_then_retry_until([]))
            .padded()
            .repeated()
    }

    /// Generate the project
    fn gen_project(p: &[Script]) -> Project {
        let mut proj = Project { variables: vec![] };

        for v in p {
            match v {
                Script::Define { typ, name, val: _ } => {
                    if typ == "var" {
                        proj.variables.push(Variable {
                            name: name.to_string(),
                            typ: 8003,
                            object_id_string: giv_me_uuid(),
                        })
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
