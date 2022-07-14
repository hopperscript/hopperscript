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
            //val: Option<String>,
        },
        Str(String),
        Loop(Vec<Self>),
    }

    /// The main compile fn
    ///
    /// Just throw a string that needs to be compiled
    ///
    /// I mean a `str`
    pub fn compile(s: &str) -> Result<Vec<Script>, Vec<Cheap<char>>> {
        let a = ast().parse(s);
        a
    }

    fn ast() -> impl Parser<char, Vec<Script>, Error = Cheap<char>> {
        let stri = just::<_, _, Cheap<char>>('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>();

        let var = just("var")
            .then(stri.padded())
            .padded()
            .map(|(a, b)| Script::Define {
                typ: a.to_string(),
                name: b,
            });
        let obj = just("object")
            .then(stri.padded())
            .padded()
            .map(|(a, b)| Script::Define {
                typ: a.to_string(),
                name: b,
            });

        let def = just("define").ignore_then(var.or(obj));

        def.recover_with(skip_then_retry_until([]))
            .padded()
            .repeated()
    }

    fn gen_project(p: &[Script]) -> Project {
        let mut proj = Project { variables: vec![] };

        for v in p {
            match v {
                Script::Define { typ, name } => {
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
