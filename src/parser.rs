use crate::tokens;
use crate::node;
use crate::parser_error;

pub fn parsing(tokens: Vec<tokens::Token>)->Result<Vec<node::Node>, parser_error::ParserError>{
    let mut node_list:Vec<node::Node>=Vec::new();
    let mut index=0;
    let mut token;
    while index<tokens.len(){
        token=&tokens[index];
        if is_a_keyword(token){
            match token.kind{
                tokens::TokenType::Pop=>{
                    
                }
                _=>{
                    let error=parser_error::ParserError{
                    kind: parser_error::ErrorType::KeyWorldExpected,
                    line: token.line,
                    };
                    return Err(error);
                }
            };
        }
        else{
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::KeyWorldExpected,
                line: token.line,
            };
            return Err(error);
        }
    }

    Ok(node_list)
}
fn is_a_keyword(token:&tokens::Token)->bool{
    return token.kind>=tokens::TokenType::Pop&&token.kind<=tokens::TokenType::Read;
}