mod define;
mod typess;
mod types;
mod getdata;

/// Main module for the compiler
pub mod compiler {

    use nom::character::complete::{newline, space0};
    use nom::multi::separated_list0;
    use nom::sequence::preceded;
    use uuid::Uuid;

    use crate::define::define;
    use crate::types::{Variable, Project};
    use crate::getdata;


    fn giv_me_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// The main compile fn
    /// 
    /// Just throw a string that needs to be compiled
    /// 
    /// I mean a `str`
    pub fn compile(input: &str) -> Project {
        getdata::init_block_data();

        let mut project = Project {
            variables: vec![]
        };

        let (_, defs) = separated_list0(newline, preceded(space0, define))(input).unwrap();
        
        for v in defs {
            match v.1 {
                0 => project.variables.push(
                    Variable { name: v.0.to_string(), typ: 8003, object_id_string: giv_me_uuid() }
                ),

                _ => {}
            }
        }

        project
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
