use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use crate::tokenizer::Tokenizer;
use crate::expr::AST;

pub mod tokenizer;
pub mod token;
pub mod expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            if !file_contents.is_empty() {
                let mut tokenizer = Tokenizer::new();

                let result = tokenizer.scan(file_contents.clone());
                tokenizer.print_tokens();

                exit(result);
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "parse" => {
            if !file_contents.is_empty() {
                let mut tokenizer = Tokenizer::new();
                let result = tokenizer.scan(file_contents.clone());
                let tokens = tokenizer.get_tokens();

                if (result != 0) {
                    exit(result);
                }

                let mut ast = AST::new(tokens); 
                ast.parse_tree(true);
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }

        }
        "evaluate" => {
            if !file_contents.is_empty() {
                let mut tokenizer = Tokenizer::new();
                let result = tokenizer.scan(file_contents.clone());
                let tokens = tokenizer.get_tokens();

                if (result != 0) {
                    exit(result);
                }

                let mut ast = AST::new(tokens); 
                ast.parse_tree(false);

                let exprs = ast.export_exprs();

                for expr in exprs {
                    let val = expr.accept();
                    println!("{}", val);
                }
                
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
