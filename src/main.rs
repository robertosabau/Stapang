use std::env;
use std::fs;

use crate::lexer_error::ErrorType::InvalidCharacter;
use crate::lexer_error::ErrorType::InvalidIndentifier;
use crate::lexer_error::ErrorType::UnclosedString;
use crate::lexer_error::LexerError;
mod tokens;
mod lexer;
mod lexer_error;
mod node;
mod parser;
mod parser_error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()<2{
        std::process::exit(1);
    }

    let file_route = &args[1];

    match fs::read_to_string(file_route) {
        Ok(code) => {
            let _tokens: Vec<tokens::Token>;
            match lexer::tokenize(code){
            Ok(tokens)=>{
                match parser::parsing(tokens){
                    Ok(_nodes)=>{

                    },
                    Err(problem)=>{
                        
                    }
                }
            },
            Err(problem)=>{
                show_lexer_error(problem);
            }
        };}
        Err(error) => {
            eprintln!("Has ocurred an error: {}",error);
            std::process::exit(1);
        }
    }

    println!("Hello, world!");
}

fn show_lexer_error(error:LexerError){
    match error.kind{
        UnclosedString=>{
            eprintln!("Syntax Error: Unclosed String found in line {}",error.line);
        },
        InvalidCharacter=>{
            eprintln!("Syntax Error: A invalid caracter found in line {}",error.line);
        }
        InvalidIndentifier=>{
            eprintln!("Syntax Error: A invalid identifier found in line {}",error.line);
        }
    }
}