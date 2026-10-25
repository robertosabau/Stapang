use crate::tokens;
use crate::node;
use crate::parser_error;
use crate::tokens::TokenType;
use crate::value;
use crate::value::ValueType;

pub fn parsing(tokens: Vec<tokens::Token>)->Result<Vec<node::Node>, parser_error::ParserError>{
    let mut node_list:Vec<node::Node>=Vec::new();
    let mut index=0;
    let mut token;
    while index<tokens.len(){
        token=tokens[index].clone();
        match generate_node(&mut token,&tokens,&mut index){
            Ok(node)=>{
                node_list.push(node);
            }
            Err(error)=>{
                return Err(error);
            }
        }
    }
    Ok(node_list)
}

fn generate_node(mut token:&mut tokens::Token,tokens: &Vec<tokens::Token>,mut index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    match token.kind{
            tokens::TokenType::Pop=>{
                match is_pop(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Push=>{
            },
            tokens::TokenType::Print | tokens::TokenType::Read | tokens::TokenType::Add 
            | tokens::TokenType::Sub | tokens::TokenType::Mult | tokens::TokenType::Div
            | tokens::TokenType::Equ | tokens::TokenType::Big | tokens::TokenType::Low
            | tokens::TokenType::BigEqu| tokens::TokenType::LowEqu | tokens::TokenType::Not
            | tokens::TokenType::And | tokens::TokenType::Or=>{
                match generic_comprobation(&token.kind.clone(), &mut token, &tokens, &mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::If=>{
                match is_if(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    }
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::While=>{
                match is_while(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    }
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Var=>{
                match is_var(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Arr=>{
                match is_arr(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Save=>{
                match is_save(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            _=>{
                let error=parser_error::ParserError{
                    kind: parser_error::ErrorType::KeyWorldExpected,
                    line: token.line,
                };
                return Err(error);
            }
        };
    let node=node::Node{
        kind: node::NodeType::VoidNode,
        tokens: Vec::new()
    };
    *index+=1;
    Ok(node)
}

fn is_pop(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let num;
    let name;
    //Checking if their is an identifier to get where to put the value of the stack
    if !check_if_is_token(tokens::TokenType::Ide,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    name=token.value.clone();
    //Checking if their is the identifier is for the case of an position in a array
    if check_if_is_token(tokens::TokenType::OBR,token, tokens, index, &mut node_tokens){
        
    }
    //Cheching case for arrays
    if token.kind!=tokens::TokenType::OBR{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenBracketExpected ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::NumberExpected,
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
    *index+=1;
    num=token.value.clone();
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected ,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::CBR{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if the sentence is closed with a semicolon, the end of the sentence
    if !check_if_is_token(tokens::TokenType::EOL,token, tokens, index, &mut node_tokens){
         let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    node=node::Node{
        kind: node::NodeType::Popnode{array:true,position:num.parse().unwrap(),name: name},
        tokens: node_tokens,
    };
    return Ok(node);
}
/*
pop name;
pop name[number];
*/
fn generic_comprobation(kind: &tokens::TokenType,token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let node_kind;
    //Checking if the sentence is closed with a semicolon, the end of the sentence
    if !check_if_is_token(tokens::TokenType::EOL,token, tokens, index, &mut node_tokens){
         let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    match *kind{
        tokens::TokenType::Print=>{
            node_kind=node::NodeType::PrintNode;
        }
        tokens::TokenType::Read=>{
            node_kind=node::NodeType::ReadNode;
        }
        tokens::TokenType::Add=>{
            node_kind=node::NodeType::AddNode;
        }
        tokens::TokenType::Sub=>{
            node_kind=node::NodeType::SubNode;
        }
        tokens::TokenType::Mult=>{
            node_kind=node::NodeType::MultNode;
        }
        tokens::TokenType::Mod=>{
            node_kind=node::NodeType::ModNode;
        }
        tokens::TokenType::Div=>{
            node_kind=node::NodeType::DivNode;
        }
        tokens::TokenType::Equ=>{
            node_kind=node::NodeType::EquNode;
        }
        tokens::TokenType::Big=>{
            node_kind=node::NodeType::BigNode;
        }
        tokens::TokenType::Low=>{
            node_kind=node::NodeType::LowNode;
        }
        tokens::TokenType::BigEqu=>{
            node_kind=node::NodeType::BigEquNode;
        }
        tokens::TokenType::LowEqu=>{
            node_kind=node::NodeType::LowEquNode;
        }
        tokens::TokenType::Not=>{
            node_kind=node::NodeType::NotNode;
        }
        tokens::TokenType::And=>{
            node_kind=node::NodeType::AndNode;
        }
        tokens::TokenType::Or=>{
            node_kind=node::NodeType::OrNode;
        }
        _=>{
            let error=parser_error::ParserError{
            kind: parser_error::ErrorType::KeyWorldExpected,
            line: token.line,
        };
        return Err(error);
        }
    }
    node_tokens.push(tokens[*index].clone());
    node=node::Node{
        kind: node_kind,
        tokens: node_tokens,
    };
    Ok(node)
}
/*
keywold;
*/
fn is_save(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let value:value::Value;
    *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::BooleanExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::True&&token.kind!=tokens::TokenType::False{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::BooleanExpected ,
            line: token.line,
        };
        return Err(error);
    }
    let val:String=match token.kind{
        tokens::TokenType::True => String::from("True"),
        tokens::TokenType::False => String::from("False"),
        _ => String::from("Error"),
    };
    //Checking if the sentence is closed with a semicolon, the end of the sentence
    if !check_if_is_token(tokens::TokenType::EOL,token, tokens, index, &mut node_tokens){
         let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    value=value::Value{
        kind: value::ValueType::Boolean,
        value: val,
    };
    node=node::Node{
        kind: node::NodeType::SaveNode { value:value },
        tokens: node_tokens,
    };
    Ok(node)
}
/*
save boolean;
*/
fn is_var(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let name:String;
    *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::Ide{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    name=token.value.clone();
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::COL{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected,
            line: token.line,
        };
        return Err(error);
    }
    let kind:ValueType;
    match token.kind{
        TokenType::NumType=>kind=ValueType::Number,
        TokenType::StrType=>kind=ValueType::Text,
        TokenType::BoolType=>kind=ValueType::Boolean,
        TokenType::AnyType=>kind=ValueType::Any,
        _=>{
            let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected ,
            line: token.line,
        };
        return Err(error);
        }
    }
    //Checking if the sentence is closed with a semicolon, the end of the sentence
    if !check_if_is_token(tokens::TokenType::EOL,token, tokens, index, &mut node_tokens){
         let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    *token = tokens[*index].clone();
    node=node::Node{
        kind:node::NodeType::VarNode { name:name,kind:kind },
        tokens:node_tokens
    };
    Ok(node)
}
/*
var name:kind;
*/
fn is_arr(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let name:String;
    *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::Ide{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    name=token.value.clone();
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenBracketExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::OBR{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenBracketExpected ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::Num{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    let num=token.value.clone();
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::CBR{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::COL{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected,
            line: token.line,
        };
        return Err(error);
    }
    let kind:ValueType;
    match token.kind{
        TokenType::NumType=>kind=ValueType::Number,
        TokenType::StrType=>kind=ValueType::Text,
        TokenType::BoolType=>kind=ValueType::Boolean,
        TokenType::AnyType=>kind=ValueType::Any,
        _=>{
            let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected ,
            line: token.line,
        };
        return Err(error);
        }
    }
    //Checking if the sentence is closed with a semicolon, the end of the sentence
    if !check_if_is_token(tokens::TokenType::EOL,token, tokens, index, &mut node_tokens){
         let error=parser_error::ParserError{
            kind: parser_error::ErrorType::SemiColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    *token = tokens[*index].clone();
    node=node::Node{
        kind:node::NodeType::ArrNode { name:name, positions:num,kind:kind },
        tokens:node_tokens
    };
    Ok(node)
}
/*
arr name[number]:kind;
*/
fn is_while(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let mut nodes:Vec<node::Node>=Vec::new();
    *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::OCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    while *index<tokens.len()&&token.kind!=tokens::TokenType::CCB{
        match generate_node(token, tokens, index){
            Ok(list)=>{
                nodes.push(list);
            }
            Err(error)=>{
                return Err(error);
            }
        }
        if *index<tokens.len(){
            node_tokens.push(tokens[*index].clone());
            *token = tokens[*index].clone();
        }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    }
    if token.kind!=tokens::TokenType::CCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    node=node::Node {
        kind: node::NodeType::WhileNode { instructions: nodes }, 
        tokens: node_tokens
    };
    Ok(node)
}
/*
while{
    code
}
*/
fn is_if(token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize)->Result<node::Node,parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let mut nodes:Vec<node::Node>=Vec::new();
    *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::OCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    while *index<tokens.len()&&token.kind!=tokens::TokenType::CCB{
        match generate_node(token, tokens, index){
            Ok(list)=>{
                nodes.push(list);
            }
            Err(error)=>{
                return Err(error);
            }
        }
        if *index<tokens.len(){
            node_tokens.push(tokens[*index].clone());
            *token = tokens[*index].clone();
        }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    }
    if token.kind!=tokens::TokenType::CCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    node_tokens.push(tokens[*index].clone());
    let mut alternative_node:Vec<node::Node>=Vec::new();
    if tokens[*index+1].kind==tokens::TokenType::Else{
        *index+=1;
    //Checking if their is an value
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    if token.kind!=tokens::TokenType::OCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
    }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    while *index<tokens.len()&&token.kind!=tokens::TokenType::CCB{
        match generate_node(token, tokens, index){
            Ok(list)=>{
                alternative_node.push(list);
            }
            Err(error)=>{
                return Err(error);
            }
        }
        if *index<tokens.len(){
            node_tokens.push(tokens[*index].clone());
            *token = tokens[*index].clone();
        }else{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    }
    if token.kind!=tokens::TokenType::CCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    }
    node=node::Node {
        kind: node::NodeType::IfNode { instructions: nodes, else_instructions: alternative_node }, 
        tokens: node_tokens
    };
    Ok(node)
}
/*
if {
    code
}
[else {
    code
}]
*/
//Function to check if their is the token we want
fn check_if_is_token(kind:tokens::TokenType,token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index, node_tokens)&&token.kind!=kind{
        return false;
    }
    true
}
//Function to check if their is another token after the actual
fn there_is_a_token(token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    *index+=1;
    if *index<tokens.len(){
        node_tokens.push(tokens[*index].clone());
        *token = tokens[*index].clone();
        return true;
    }else{
        return false;
    }
}
//Function to check if their is a type
fn check_if_is_a_type(token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index, node_tokens)&&(token.kind!=tokens::TokenType::NumType||token.kind!=tokens::TokenType::StrType||token.kind!=tokens::TokenType::BoolType){
        return false;
    }
    true
}