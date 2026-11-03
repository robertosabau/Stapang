use std::env;
use std::fs;
//use std::time::Instant;
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
mod array;
mod calculator;
fn main() {
    /*
    Timer for check the time to make an instruction
    let start = Instant::now();
    let mut sum: u64 = 0;
    for i in 0..1000000 {
        sum += i;
    }
    let duration = start.elapsed();
    */
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
                        match evaluator::evaluation(_nodes){
                            Ok(_) =>{}
                            Err(problem) =>{ 
                                show_evaluator_error(problem);
                                std::process::exit(1);}
                            }
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
    //println!("Execution time: {} ns", duration.as_nanos()); // To check the time to make an code
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
fn show_evaluator_error(error:evaluator_error::EvaluatorError){
    match error.kind {
        evaluator_error::ErrorType::BooleanExpected=>{
            eprintln!("Runtime Error: an Boolean was expected in line {}",error.line);
        }
        evaluator_error::ErrorType::StackEmpty =>{
            eprintln!("Runtime Error: the Stack was empty when tried to take out a value in line {}",error.line)
        }
        evaluator_error::ErrorType::VariableNotDefined { name } =>{
            eprintln!("Runtine Error: the variable {} was not defined when tried to used it in line {}",name,error.line)
        }
        evaluator_error::ErrorType::ArrayNotDefined { name } =>{ 
            eprintln!("Runtine Error: the array {} was not defined when tried to used it in line {}",name,error.line)
        }
        evaluator_error::ErrorType::InternarError =>{
            eprintln!("Inexpected Error: has ocurred an error that was not supossed to happend in the internar code")
        }
        evaluator_error::ErrorType::OutOfBound =>{
            eprintln!("Runtime Error: intented to access a position out of bounds in line {}",error.line)
        }
        evaluator_error::ErrorType::InvalidType =>{
            eprintln!("Runtime Error: invalid type used in line {}",error.line)
        }
        evaluator_error::ErrorType::DivisionByZero => {
            eprintln!("Runtime Error: division by zero made in line {}",error.line)
        }
        evaluator_error::ErrorType::ReadError => {
            eprintln!("Runtime Error: was not posible to read in line {}",error.line)
        }
        evaluator_error::ErrorType::CannotConvertToNumber => {
            eprintln!("Runtime Error: tried to convert an invalid str to a number in line {}",error.line)
        }
        evaluator_error::ErrorType::InvalidValue => {
            eprintln!("Runtime Error: tried to set a value invalid in a variable/array in line {}",error.line)
        }
        evaluator_error::ErrorType::VariableNotInicializated { name } =>{
            eprintln!("Runtine Error: the variable {} was not inicializated when tried to used it in line {}",name,error.line)
        }
        evaluator_error::ErrorType::ArrayNotInicializated { name,position } =>{
            eprintln!("Runtine Error: the position {} in the array {} was not inicializated when tried to used it in line {}",position,name,error.line)
        }
    }
}