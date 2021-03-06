use std::io::prelude::*;

mod lexer;
mod parser;
mod computer;

use lexer::*;
use parser::*;
use computer::*;

fn print_help() {
    println!("Usage: rsc [OPTIONS]\n\nOPTIONS\n\tast\tPrint the AST generated by the expression entered\n\thelp\tThis help screen");
}

fn main() {
    let mut print_ast = false;

    for arg in std::env::args().collect::<Vec<String>>()[1..].iter() {
        match &arg[..] {
            "ast" => print_ast = true,
            "help" => {
                print_help();
                std::process::exit(0);
            }
            opt => {
                println!("Unknown option {:?} entered\n", opt);
                print_help();
                std::process::exit(1);
            }
        }
    }

    let mut buffer = String::new();

    loop {
        print!(">");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer == "quit" || &buffer == "exit" {
            break;
        }

        match tokenize(&buffer) {
            Ok(tokens) => {
		        //println!("{:?}", tokens);
                match parse(&tokens) {
                    Ok(ast) => {
                        if print_ast {
                            println!("{:#?}", ast);
                        }
                        println!("{}", compute(&ast));
                    }
                    Err(err) => {
                        println!("Parser error: {:?}", err);
                    }
                }
            }
            Err(err) => {
                println!("Lexer error: {:?}", err);
            }
        }

        //buffer = String::new();
        buffer.clear();
    }
}
