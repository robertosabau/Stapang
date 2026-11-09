use crate::tokens;
use crate::node;
use crate::parser_error;
use crate::tokens::TokenType;
use crate::value;
use crate::value::ValueType;
//Funtion to start the parsing
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
//Function to generate a node for instructions
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
                match is_push(&mut token,&tokens,&mut index){
                    Ok(node)=>{
                        *index+=1;
                        return Ok(node);
                    },
                    Err(error)=>{
                        return Err(error);
                    }
                }
            },
            tokens::TokenType::Read=>{
                match is_read(token, tokens, &mut index){
                    Ok(node) =>{
                        *index+=1;
                        return Ok(node);},
                    Err(error) => return Err(error),
                }
            }
            tokens::TokenType::Print | tokens::TokenType::Add 
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
            _=>{
                let error=parser_error::ParserError{
                    kind: parser_error::ErrorType::KeyWorldExpected,
                    line: token.line,
                };
                return Err(error);
            }
        };
}
//Function to detect an pop
fn is_pop(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let mut num:String=String::from("0");
    let name;
    let mut is_array = false;
    //Checking if their is an identifier to get where to put the value of the stack
    if !check_if_is_token(tokens::TokenType::Ide,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    name=token.value.clone();
    //Checking if there is the identifier is for an array
    if check_if_is_token(tokens::TokenType::OBR,token, tokens, index, &mut node_tokens){
        is_array=true;
        //Checking if there is the number of the position in the array
        if !check_if_is_token(tokens::TokenType::Num,token,tokens,index,&mut node_tokens){
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::NumberExpected,
                line: token.line,
            };
            return Err(error);
        }
        num=token.value.clone();
        //Checking if there is the closing bracked
        if !check_if_is_token(tokens::TokenType::CBR,token,tokens,index,&mut node_tokens){
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::CloseBrakectExpected,
                line: token.line,
            };
            return Err(error);
        }
    }
    else {
        *index -= 1;
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
        kind: node::NodeType::Popnode{array:is_array,position:num.parse().unwrap(),name: name},
        tokens: node_tokens,
    };
    return Ok(node);
}
/*
pop name;
pop name[number];
*/
fn is_push(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::NodeType;
    let final_node:node::Node;
    //Checking if there is an value to push
    if !there_is_a_value(token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line:token.line,
        };
        return Err(error);
    }
    //Match to know which value we have
    match token.kind{
        tokens::TokenType::True =>{
            node=node::NodeType::Pushnode { array: false, name: String::from("Null"), position:Some(0), value:value::Value::set_as_true()}
        }
        tokens::TokenType::False =>{
            node=node::NodeType::Pushnode { array: false, name: String::from("Null"), position:Some(0), value:value::Value::set_as_false() }
        },
        tokens::TokenType::Num=>{
            node=node::NodeType::Pushnode { array: false, name: String::from("Null"), position:Some(0), value:value::Value::set_as_number(token.value.clone())}
        }
        tokens::TokenType::Str=>{
            node=node::NodeType::Pushnode { array: false, name: String::from("Null"), position:Some(0), value:value::Value::set_as_text(token.value.clone())}
        }
        tokens::TokenType::Ide=>{
            //Saving the name of the identifier
            let name = token.value.clone();
            //Checking if there is the identifier is for an array
            if check_if_is_token(tokens::TokenType::OBR,token, tokens, index, &mut node_tokens){
                //Checking if there is the number of the position in the array
                if !check_if_is_token(tokens::TokenType::Num,token,tokens,index,&mut node_tokens){
                    let error=parser_error::ParserError{
                        kind: parser_error::ErrorType::NumberExpected,
                        line: token.line,
                    };
                    return Err(error);
                }
                let num=token.value.clone();
                //Checking if there is the closing bracked
                if !check_if_is_token(tokens::TokenType::CBR,token,tokens,index,&mut node_tokens){
                    let error=parser_error::ParserError{
                        kind: parser_error::ErrorType::CloseBrakectExpected,
                        line: token.line,
                    };
                    return Err(error);
                }
                node=node::NodeType::Pushnode { array: true, name:name, position:num.parse::<usize>().ok(), value:value::Value::set_as_null()}
            }
            else{
                *index-=1;
                node=node::NodeType::Pushnode { array: false, name:name, position:Some(0), value:value::Value::set_as_null()}
            }
        }
        _=>{
            let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line:token.line,
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
    final_node=node::Node { kind:node, tokens: node_tokens };
    Ok(final_node)
}
/*
push bool;
push name;
push name[number];
push number;
push String;
*/
//Function for all the instruction without parameters
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
    //Match to get witch instruction we have
    match *kind{
        tokens::TokenType::Print=>{
            node_kind=node::NodeType::PrintNode;
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
fn is_var(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize)->Result<node::Node, parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let name:String;
    let mut kind:ValueType=ValueType::Null;
    //Checking if their is an identifier
    if !check_if_is_token(tokens::TokenType::Ide,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    }
    name=token.value.clone();
    //Checking if there is an colon to enter the type of the variable
    if !check_if_is_token(tokens::TokenType::COL,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if there is an type for the array
    if !there_is_a_type(&mut kind, token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected,
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
    let num:String;
    let mut kind:ValueType=ValueType::Null;
    //Checking if their is an identifier
    if !check_if_is_token(tokens::TokenType::Ide,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted ,
            line: token.line,
        };
        return Err(error);
    };
    name=token.value.clone();
    //Checking if their is an open bracket to set the number of position it will have the array
    if !check_if_is_token(tokens::TokenType::OBR,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenBracketExpected ,
            line: token.line,
        };
        return Err(error);
    };
    //Checking if there is the number of the positions of the array
    if !check_if_is_token(tokens::TokenType::Num,token,tokens,index,&mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::NumberExpected,
            line: token.line,
        };
        return Err(error);
    }
    num=token.value.clone();
    //Checking if there is the closing bracked
    if !check_if_is_token(tokens::TokenType::CBR,token,tokens,index,&mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseBrakectExpected,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if there is an colon to enter the type of the variable
    if !check_if_is_token(tokens::TokenType::COL,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ColonExpected ,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if there is an type for the array
    if !there_is_a_type(&mut kind, token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected,
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
    //Checking if their is an open curvy bracked to start the secuence of sentences of the while
    if !check_if_is_token(tokens::TokenType::OCB,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected ,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if there is an token
    if !there_is_a_token(token, tokens, index, &mut node_tokens){
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
        *index-=1;
        if !there_is_a_token(token, tokens, index, &mut node_tokens){
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
    //Checking if their is an open curvy bracked to start the secuence of sentences of the while
    if !check_if_is_token(tokens::TokenType::OCB,token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::OpenCurvyBrackedExpected ,
            line: token.line,
        };
        return Err(error);
    }
    //Checking if there is an token
    if !there_is_a_token(token, tokens, index, &mut node_tokens){
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
        *index-=1;
        if !there_is_a_token(token, tokens, index, &mut node_tokens){
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
    //Checking if there is an Else for the alternative path
    if check_if_is_token(tokens::TokenType::Else,token, tokens, index, &mut node_tokens){
        //Checking if there is an open curvy bracket to start  lecture of instruction for the alternative path
        if !check_if_is_token(tokens::TokenType::OCB, token, tokens, index, &mut node_tokens){
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::OpenCurvyBrackedExpected,
                line: token.line,
            };
            return Err(error);
        }
        //Checking if there is an token to start the while loop
        if !there_is_a_token(token, tokens, index, &mut node_tokens){
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
                line: token.line,
            };
            return Err(error);
        }
        //While loop to get the istruction for the alternative path
        while *index<tokens.len()&&token.kind!=tokens::TokenType::CCB{
            match generate_node(token, tokens, index){
                Ok(list)=>{
                    alternative_node.push(list);
                }
                Err(error)=>{
                    return Err(error);
                }
            }
            *index-=1;
            if !there_is_a_token(token, tokens, index, &mut node_tokens){
                let error=parser_error::ParserError{
                    kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
                    line: token.line,
                };
                return Err(error);
            }
        }
    //Checking if there is an close curve bracked to end the lecture correctly
    if token.kind!=tokens::TokenType::CCB{
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::CloseCurvyBrackedExpected,
            line: token.line,
        };
        return Err(error);
    }
    }else{
        alternative_node.push(node::Node{kind:node::NodeType::VoidNode,tokens:node_tokens.clone()})
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
//Function to check if there is the token we want
fn check_if_is_token(kind:tokens::TokenType,token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index, node_tokens)||token.kind!=kind{
        return false;
    }
    true
}
//Function to check if there is another token after the actual
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
//Function to check if there is a type of data
fn there_is_a_type(kind:&mut ValueType,token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index,node_tokens){
        return false;
    }
    //Checking if the tokens is a type of data
    match token.kind{
        TokenType::NumType=>*kind=ValueType::Number,
        TokenType::StrType=>*kind=ValueType::Text,
        TokenType::BoolType=>*kind=ValueType::Boolean,
        TokenType::AnyType=>*kind=ValueType::Any,
        _=>{
            return false;
        }
    }
    true
}
//Function to check if there is an value to push
fn there_is_a_value(token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index,node_tokens){
        return false;
    }
    //Checking if the tokens is a value can you get
    match token.kind{
        TokenType::Num|TokenType::Ide|TokenType::True|TokenType::False|TokenType::Str=>return true,
        _=>return false,
    }
}
//Function for the read instruction
fn is_read(token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize)->Result<node::Node,parser_error::ParserError>{
    let mut node_tokens:Vec<tokens::Token>=Vec::new();
    let node: node::Node;
    let mut kind:ValueType=ValueType::Null;
    //Checking if there is an type for the array
    if !there_is_a_type(&mut kind, token, tokens, index, &mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::TypeExpected,
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
    *token = tokens[*index].clone();
    node=node::Node{
        kind:node::NodeType::ReadNode { format: kind },
        tokens:node_tokens
    };
    Ok(node)
}