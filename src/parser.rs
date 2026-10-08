use crate::parser_error::ParserError;
use crate::tokens;
use crate::node;
use crate::parser_error;

pub fn parsing(tokens: Vec<tokens::Token>)->Result<Vec<node::Node>, parser_error::ParserError>{
    let mut node_list:Vec<node::Node>=Vec::new();


    let x=2;
    if x==2{
        Err::ParserError(ParserError{kind: parser_error::ErrorType::UnclosedString,line: 2});
    }

    Ok(node_list)
}