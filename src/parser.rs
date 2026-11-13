use crate::tokens;
use crate::node;
use crate::parser_error;
use crate::tokens::TokenType;
use crate::value;
use crate::value_source;
use crate::variable;
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
            | tokens::TokenType::Equ | tokens::TokenType::Greater | tokens::TokenType::Less
            | tokens::TokenType::GreaterEqu| tokens::TokenType::LessEqu | tokens::TokenType::Not
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
    let kind:value_source::Identifier;
    //match to get the identifier
    match get_identifier(token, tokens, index, &mut node_tokens){
        Ok(ide) => kind=ide,
        Err(error) => return Err(error),
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
        kind: 
        node::NodeType::Popnode{ide:kind},
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
            node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_true())}
        }
        tokens::TokenType::False =>{
            node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_false())}
        },
        tokens::TokenType::Num=>{
            if token.value.contains("."){
                node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_float(token.value.clone().parse::<f64>().unwrap()))}
            }
            else{
                let num=token.value.clone().parse::<i64>().unwrap();
                if num<0{
                    node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_int(num))};
                }
                else{
                    node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_uint(usize::try_from(num).unwrap()))};
                }
            }
        }
        tokens::TokenType::Str=>{
            node=node::NodeType::Pushnode { value: value_source::ValueSource::Literal(value::Value::set_as_text(token.value.clone()))}
        }
        tokens::TokenType::Ide=>{
            *index-=1;
            //match to get the identifier
            match get_identifier(token, tokens, index, &mut node_tokens){
                Ok(ide) => node=node::NodeType::Pushnode { value: value_source::ValueSource::Identifier(ide) },
                Err(error) => return Err(error),
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
        tokens::TokenType::Greater=>{
            node_kind=node::NodeType::GreaterNode;
        }
        tokens::TokenType::Less=>{
            node_kind=node::NodeType::LessNode;
        }
        tokens::TokenType::GreaterEqu=>{
            node_kind=node::NodeType::GreaterEquNode;
        }
        tokens::TokenType::LessEqu=>{
            node_kind=node::NodeType::LessEquNode;
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
    let mut kind=variable::VariableType::Any;
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
    let mut kind=variable::VariableType::Any;
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
fn there_is_a_type(kind:&mut variable::VariableType,token:&mut tokens::Token,tokens:&Vec<tokens::Token>,index: &mut usize,node_tokens:&mut Vec<tokens::Token>)->bool{
    if !there_is_a_token(token, tokens, index,node_tokens){
        return false;
    }
    //Checking if the tokens is a type of data
    match token.kind{
        TokenType::FloatType=>*kind=variable::VariableType::Float,
        TokenType::StrType=>*kind=variable::VariableType::Text,
        TokenType::BoolType=>*kind=variable::VariableType::Boolean,
        TokenType::AnyType=>*kind=variable::VariableType::Any,
        TokenType::IntType=>*kind=variable::VariableType::Int,
        TokenType::UIntType=>*kind=variable::VariableType::UInt,
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
    let mut kind:variable::VariableType=variable::VariableType::Any;
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
//Function to get a identifier
fn get_identifier(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize,mut node_tokens:&mut Vec<tokens::Token>)->Result<value_source::Identifier,parser_error::ParserError>{
    //Checking if is an identifier
    if !check_if_is_token(tokens::TokenType::Ide,token,tokens,index,&mut node_tokens){
        let error=parser_error::ParserError{
            kind: parser_error::ErrorType::ValueExcepted,
            line: token.line,
        };
        return Err(error);
    }
    //Saving name for identifier
    let name=token.value.clone();
    //Checking if the 
    if check_if_is_token(tokens::TokenType::OBR,token, tokens, index, &mut node_tokens){
        let source;
        //Getting posiiton of array
        match get_position_array(token, tokens, index, &mut node_tokens){
            Ok(sour) =>source=sour,
            Err(error) => return Err(error),
        }
        //Checking if there is the closing bracked
        if !check_if_is_token(tokens::TokenType::CBR,token,tokens,index,&mut node_tokens){
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::CloseBrakectExpected,
                line: token.line,
            };
            return Err(error);
        }
        //Return identifier
        return Ok(value_source::Identifier::Array { name, position:source });
    }
    else {
        *index -= 1;
        return Ok(value_source::Identifier::Variable(name));
    }

}
//Function to get position in a array
fn get_position_array(token:&mut tokens::Token,tokens: &Vec<tokens::Token>,index: &mut usize,mut node_tokens:&mut Vec<tokens::Token>)->Result<Box<value_source::ValueSource>,parser_error::ParserError>{
    //Checking if there is the number of the position in the array
    if !check_if_is_token(tokens::TokenType::Num,token,tokens,index,&mut node_tokens){
        *index-=1;
        //Checking if is an identifiier
        match get_identifier(token, tokens, index, node_tokens){
            Ok(identifier) => return Ok(Box::new(value_source::ValueSource::Identifier(identifier))),
            Err(error) => return Err(error),
        }
        
    }
    //Checking if is a valid number 
    match token.value.parse::<usize>(){
        Ok(num) => return Ok(Box::new(value_source::ValueSource::Literal(value::Value::set_as_uint(num)))),
        Err(_) =>{
            let error=parser_error::ParserError{
                kind: parser_error::ErrorType::NumberExpected,
                line: token.line,
            };
            return Err(error);}
    }
}