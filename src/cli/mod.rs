use std::fs;
use lib::compiler::compile;
use std::env;

pub fn read_file(path: &String) -> String {
    let val = fs::read_to_string(path).expect("Error reading file.");
    val
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
            println!("{:#?}", compile(read_file(&args[2]).as_str()));
        } else {
            // unknown
            println!("Unknown option.\nUse \"{} help\" for help", COMMAND_NAME)
        }
    }
}
