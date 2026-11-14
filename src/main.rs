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
mod value_source;
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
            eprintln!("Syntax Error: Unterminated string in line {}",error.line);
        },
        lexer_error::ErrorType::InvalidCharacter=>{
            eprintln!("Syntax Error: Invalid character found in line {}",error.line);
        }
        lexer_error::ErrorType::InvalidIndentifier=>{
            eprintln!("Syntax Error: Invalid identifier found in line {}",error.line);
        }
    }
}
fn show_parser_error(error:ParserError){
    match error.kind{
        parser_error::ErrorType::KeyWorldExpected=>{
            eprintln!("Syntax Error: Keyword expected in line {}",error.line);
        }
        parser_error::ErrorType::ValueExcepted=>{
            eprintln!("Syntax Error: Value expected in line {}",error.line);
        }
        parser_error::ErrorType::SemiColonExpected=>{
            eprintln!("Syntax Error: ';' expected in line {}",error.line);
        }
        parser_error::ErrorType::OpenBracketExpected=>{
            eprintln!("Syntax Error: '[' expected in line {}",error.line);
        }
        parser_error::ErrorType::CloseBrakectExpected=>{
            eprintln!("Syntax Error: ']' expected in line {}",error.line);
        }
        parser_error::ErrorType::NumberExpected=>{
            eprintln!("Syntax Error: Number expected in line {}",error.line);
        }
        parser_error::ErrorType::OpenCurvyBrackedExpected=>{
            eprintln!("Syntax Error: '{{' expected in line {}",error.line);
        }
        parser_error::ErrorType::CloseCurvyBrackedExpected=>{
            eprintln!("Syntax Error: '}}' expected in line {}",error.line);
        }
        parser_error::ErrorType::ColonExpected=>{
            eprintln!("Syntax Error: ':' expected in line {}",error.line);
        }
        parser_error::ErrorType::TypeExpected=>{
            eprintln!("Syntax Error: Type expected in line {}",error.line);
        }
    }
}
fn show_evaluator_error(error:evaluator_error::EvaluatorError){
    match error.kind {
        evaluator_error::ErrorType::BooleanExpected=>{
            eprintln!("Runtime Error: Boolean value expected at the top of the stack in line {}",error.line);
        }
        evaluator_error::ErrorType::StackEmpty =>{
            eprintln!("Runtime Error: Attempted to retrieve a value from an empty stack in line {}",error.line)
        }
        evaluator_error::ErrorType::VariableNotDefined { name } =>{
            eprintln!("Runtime Error: Variable '{}' is not defined in line {}",name,error.line)
        }
        evaluator_error::ErrorType::ArrayNotDefined { name } =>{ 
            eprintln!("Runtime Error: Array '{}' is not defined in line {}",name,error.line)
        }
        evaluator_error::ErrorType::InternarError =>{
            eprintln!("Internal Error: An unexpected error occurred inside the interpreter.")
        }
        evaluator_error::ErrorType::OutOfBound =>{
            eprintln!("Runtime Error: Attempted to access an array position out of bounds in line {}",error.line)
        }
        evaluator_error::ErrorType::InvalidType =>{
            eprintln!("Runtime Error: Invalid type used in line {}",error.line)
        }
        evaluator_error::ErrorType::DivisionByZero => {
            eprintln!("Runtime Error: Division by zero in line {}",error.line)
        }
        evaluator_error::ErrorType::ReadError => {
            eprintln!("Runtime Error: Unable to read from standard input in line {}",error.line)
        }
        evaluator_error::ErrorType::NumericConversionError => {
            eprintln!("Runtime Error: Failed to convert a value to a numeric type in line {}",error.line)
        }
        evaluator_error::ErrorType::InvalidValue => {
            eprintln!("Runtime Error: Attempted to assign an incompatible value to a variable or array position in line {}",error.line)
        }
        evaluator_error::ErrorType::VariableNotInicializated { name } =>{
            eprintln!("Runtime Error: Variable '{}' is not initialized in line {}",name,error.line)
        }
        evaluator_error::ErrorType::ArrayNotInicializated { name,position } =>{
            eprintln!("Runtime Error: Position {} in array '{}' is not initialized in line {}",position,name,error.line)
        }
    }
}