use std::env;

mod util;
mod repl;
mod token;
mod lexer;

const VERSION: &'static str = "v0.1 Alpha";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            repl::repl();
        }
        2 => {
            let command = args.get(1).unwrap().as_str();
            match command {
                "-v" | "--version" => {
                    version();
                },
    
                "-h" | "--help" => {
                    help();
                }
    
                _ => usage_error()
            }
        }
        3 => {
            let command = args.get(1).unwrap().as_str();
            //let file = args.get(2).unwrap();

            match command {
                "run" => {
                    
                },

                "build" => {

                }

                _ => usage_error() // todo! treat as run <file> ?
            }
        },

        _ => usage_error()
    }
}

fn usage_error() {
    println!("Usage: csimple run <file> | build <file> | (-v | --version) | (-h | --help)");
}

fn version() {
    println!("CSimple | Version {}\nMade by JuniorBecari10. All rights reserved.", VERSION);
}

fn help() {
    usage_error();

    println!("\n run <file>     | compiles and executes the source code inside the provided file.");
    println!(" build <file>   | compiles the source code inside the provided file into bytecode.");
    println!(" -v | --version | shows up the current version number of the compiler.");
    println!(" -h | --help    | shows up this help message.");
}
// todo! fazer uma regra que só lê bytecode
