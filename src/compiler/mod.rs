use nom::{
    IResult,
    sequence::delimited,
    character::complete::char,
    bytes::complete::is_not
};

mod define;

fn string(i: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(i)
}

pub fn compile(input: &str) -> IResult<&str, &str> {
    define::define(input)
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
