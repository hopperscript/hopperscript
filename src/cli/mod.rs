/// central file for the compiler
use std::fs;
use lib::compile as comp;
use std::env;

pub fn read_file(path: &String) -> String {
    let val = fs::read_to_string(path).expect("Error reading file.");
    val
}

/// main compile function
pub fn compile(code: String) {
    // only testing (uncomment)
    println!("{:?}", (comp(&code)));
}

// constants (lol)
const LANG_NAME: &str = "hopperscript";
const HELP_MENU: &str = "compiler

Compiles code into a Hopscotch JSON file.

OPTIONS:
	%n compile [FILENAME]
";
const COMMAND_NAME: &str = "command-name";

fn main() {
    if env::args().len() == 1 {
        // no args
        println!(
            "{} Compiler\nUse \"{} help\" for help",
            LANG_NAME, COMMAND_NAME
        );
    } else {
        let args: Vec<String> = env::args().collect();
        if args[1] == "help" {
            // help command
            println!("{} {}", LANG_NAME, HELP_MENU.replace("%n", COMMAND_NAME))
        } else if args[1] == "compile" {
            // compile
            compile(read_file(&args[2]))
        } else {
            // unknown
            println!("Unknown option.\nUse \"{} help\" for help", COMMAND_NAME)
        }
    }
}
