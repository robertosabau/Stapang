use crate::tokens;
use crate::node;
use crate::parser_error;

pub fn parsing(tokens: Vec<tokens::Token>)->Result<Vec<node::Node>, parser_error::ParserError>{
    let mut node_list:Vec<node::Node>=Vec::new();
    let mut index=0;
    let mut token;
    while index<tokens.len(){
        token=tokens[index].clone();
        match token.kind{
            tokens::TokenType::Pop=>{
                match is_pop(&mut token,&tokens,&mut index){
                    Ok(nodo)=>{
                        node_list.push(nodo);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Push=>{

            },
            tokens::TokenType::Print=>{

            },
            tokens::TokenType::If=>{

            },
            tokens::TokenType::While=>{

            },
            tokens::TokenType::Var=>{

            },
            tokens::TokenType::Arr=>{

            },
            tokens::TokenType::Save=>{

            },
            tokens::TokenType::Read=>{

            },
            tokens::TokenType::Add=>{

            },
            tokens::TokenType::Sub=>{

            },
            tokens::TokenType::Mult=>{

            },
            tokens::TokenType::Div=>{

            },
            tokens::TokenType::Equ=>{

            },
            tokens::TokenType::Big=>{

            },
            tokens::TokenType::Low=>{

            },
            tokens::TokenType::BigEqu=>{

            },
            tokens::TokenType::LowEqu=>{
                
            },
            tokens::TokenType::Not=>{
                
            },
            tokens::TokenType::And=>{
                
            },
            tokens::TokenType::Or=>{
                
            },
            _=>{
                let error=parser_error::ParserError{
                kind: parser_error::ErrorType::KeyWorldExpected,
                line: token.line,
                };
                return Err(error);
            }
        };
    }

    Ok(node_list)
}

fn is_pop(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let node_tokens:Vec<tokens::Token>;
    let node: node::Node;
    let kind= node::NodeType::Popnode;
    let num;
    *index+=1;
    if *index<tokens.len(){
        *token=<tokens::Token as Clone>::clone(&tokens[*index]);
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::Ide{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        *token=<tokens::Token as Clone>::clone(&tokens[*index]);
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind==tokens::TokenType::EOL{
        let nodo=node::Node{
            kind: kind,
            tokens: node_tokens,
            array: false,
            position: 0,
        };
        return Ok(nodo);
    }
    if *index<tokens.len(){
        *token=<tokens::Token as Clone>::clone(&tokens[*index]);
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenBracketExpected ,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::Num{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::NumberExpected ,
            line: token.line,
        };
        return Err(error);
    }
    num=token.value;
    if *index<tokens.len(){
        *token=<tokens::Token as Clone>::clone(&tokens[*index]);
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected ,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::EOL{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected,
            line: token.line,
        };
        return Err(error);
    }

    return Ok(nodo);
}