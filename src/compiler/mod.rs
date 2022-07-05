mod define;
mod typess;
mod types;

/// Main module for the compiler
pub mod compiler {
    use nom::character::complete::{newline, space0};
    use nom::multi::separated_list0;
    use nom::sequence::preceded;

    use crate::define::define;
    use crate::types::{self, Project};

    /// The main compile fn
    /// 
    /// Just throw a string that needs to be compiled
    /// 
    /// I mean a `str`
    pub fn compile(input: &str) -> types::Project {
        let mut project = Project {
            variables: vec![]
        };

        project.variables = separated_list0(newline, preceded(space0, define))(input).unwrap().1.into_iter().map(String::from).collect();

        project
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
