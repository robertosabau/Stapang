use std::env;
use std::fs;

use crate::lexer_error::LexerError;
use crate::parser_error::ParserError;
mod tokens;
mod lexer;
mod lexer_error;
mod node;
mod parser;
mod parser_error;
mod value;
mod variable;
mod evaluator;
mod evaluator_error;

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
                        evaluator::evaluation(_nodes);
                    },
                    Err(problem)=>{
                        show_parser_error(problem);
                        std::process::exit(1);
                    }
                }
            },
            Err(problem)=>{
                show_lexer_error(problem);
                std::process::exit(1);
            }
        };}
        Err(error) => {
            eprintln!("Has ocurred an error: {}",error);
            std::process::exit(1);
        }
    }
}

fn show_lexer_error(error:LexerError){
    match error.kind{
        lexer_error::ErrorType::UnclosedString=>{
            eprintln!("Syntax Error: Unclosed String found in line {}",error.line);
        },
        lexer_error::ErrorType::InvalidCharacter=>{
            eprintln!("Syntax Error: A invalid caracter found in line {}",error.line);
        }
        lexer_error::ErrorType::InvalidIndentifier=>{
            eprintln!("Syntax Error: A invalid identifier found in line {}",error.line);
        }
    }
}
fn show_parser_error(error:ParserError){
    match error.kind{
        parser_error::ErrorType::KeyWorldExpected=>{
            eprintln!("Syntax Error: An keyworld was expeted in line {}",error.line);
        }
        parser_error::ErrorType::ValueExcepted=>{
            eprintln!("Syntax Error: An value was expected in line {}",error.line);
        }
        parser_error::ErrorType::SemiColonExpected=>{
            eprintln!("Syntax Error: ; expected in line {}",error.line);
        }
        parser_error::ErrorType::OpenBracketExpected=>{
            eprintln!("Syntax Error: [ expected in line {}",error.line);
        }
        parser_error::ErrorType::CloseBrakectExpected=>{
            eprintln!("Syntax Error: ] expected in line {}",error.line);
        }
        parser_error::ErrorType::NumberExpected=>{
            eprintln!("Syntax Error: number expected in line {}",error.line);
        }
        parser_error::ErrorType::BooleanExpected=>{
            eprintln!("Syntax Error: boolean expected in line {}",error.line);
        }
        parser_error::ErrorType::OpenCurvyBrackedExpected=>{
            eprintln!("Syntax Error: {{ expected in line {}",error.line);
        }
        parser_error::ErrorType::CloseCurvyBrackedExpected=>{
            eprintln!("Syntax Error: }} expected in line {}",error.line);
        }
        parser_error::ErrorType::ColonExpected=>{
            eprintln!("Syntax Error: : expected in line {}",error.line);
        }
        parser_error::ErrorType::TypeExpected=>{
            eprintln!("Syntax Error: type expected in line {}",error.line);
        }
    }
}