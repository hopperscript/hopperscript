/// central file for the compiler
use std::fs;
mod grammar;

pub fn read_file(path: &String) -> String {
    let val = fs::read_to_string(path).expect("Error reading file.");
    val
}

/// main compile function
pub fn compile(code: String) {
    //only testing
    println!("{:?}", grammar::parser::lines(&code).ok().unwrap());
}
