use peg;

include!("types.rs");

peg::parser! {
    pub grammar parser() for str {
        //collection of code blocks
        
        //removed code lol
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
