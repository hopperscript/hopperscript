mod define;

/// Main module for the compiler
pub mod compiler {
    use nom::{
        IResult,
        sequence::delimited,
        character::complete::char,
        bytes::complete::is_not
    };

    use crate::define::define;

    /// for strings (`""`) I guess?
    fn string(i: &str) -> IResult<&str, &str> {
        delimited(char('"'), is_not("\""), char('"'))(i)
    }

    /// The main compile fn
    /// 
    /// Just throw a string that needs to be compiled
    /// 
    /// I mean a `str`
    pub fn compile(input: &str) -> IResult<&str, &str> {
        define(input)
    }
}

//peg::parser! {
//    pub grammar parser() for str {
//        rule string() -> String
//            = s:$("\""[str]"\"") { String::from(s) }

//        pub rule define() -> String
//            = s:$("define "$([str]+) $([str]*)) {
//                String::from(s)
//            }
//    }
//}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
