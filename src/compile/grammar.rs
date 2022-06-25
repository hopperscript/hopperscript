use peg;

include!("types.rs");

peg::parser! {
    pub grammar parser() for str {
        pub rule lines() -> Vec<String>
        = n:$((['\0'..='\x7F']))+"\n" {n.into_iter().map(|n| n.to_string()).rev().collect()}
    }
}

// for later use:

// fn to_fncall(n: String) -> Fncall {
//     Fncall { fnname: n }
// }
