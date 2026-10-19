use crate::tokens;
use crate::lexer_error;

pub fn tokenize(code: String) -> Result<Vec<tokens::Token>, lexer_error::LexerError>{
    let mut token_list: Vec<tokens::Token>=Vec::new();
    let caracteres: Vec<char> = code.chars().collect();
    let mut index=0;
    let mut _line=1;
    let mut caracter;
    let mut name: String;
    while index<caracteres.len(){
        caracter=caracteres[index];
        if is_invalid_caracter(caracter){
            let error=lexer_error::LexerError{
                kind: lexer_error::ErrorType::InvalidCharacter,
                line: _line
            };
            return Err(error);
        }
        while is_useless(&caracter, &mut _line,){
            index+=1;
            if index<caracteres.len(){
                caracter=caracteres[index];
            }
        }
        if is_special_caracter(caracter, &mut token_list,_line){
            index += 1;
            continue;
        }
        if is_comentery(caracter){
            while index<caracteres.len()&&caracter!='\n'{
                index+=1;
                if index<caracteres.len(){
                    caracter=caracteres[index];
                }
            }
            if caracter=='\n'{
                _line+=1;
            }
            index+=1;
            continue;
        }
        if caracter=='\"'{
            index+=1;
            if index<caracteres.len(){
                caracter=caracteres[index];
            }
            name=String::from("");
            while index<caracteres.len()&&caracter!='\"'{
                if caracter=='\n'{
                    _line+=1;
                }
                name.push(caracter);
                index+=1;
                if index<caracteres.len(){
                    caracter=caracteres[index];
                }
            }
            if caracter=='\"'{
                let token=tokens::Token{
                    kind:tokens::TokenType::Str,
                    line:_line,
                    value:name,
                };
                token_list.push(token);
                index += 1;
                continue;
            }
            else{
                let error=lexer_error::LexerError{
                    kind: lexer_error::ErrorType::UnclosedString,
                    line: _line
                };
                return Err(error);
            }
        }
        else{
            name=String::from("");
            while index<caracteres.len()&&!is_useless(&caracter, &mut _line){
                name.push(caracter);
                index+=1;
                if index<caracteres.len(){
                    caracter=caracteres[index];
                }
            }
                if is_number(&mut token_list,&name, _line){
                    continue;
                }
                if is_keyword(&mut token_list,&name, _line){
                    continue;
                }
                if !contains_invalid_caracter(&name){
                    let token=tokens::Token{
                        kind: tokens::TokenType::Ide,
                        line: _line,
                        value: name.clone(),
                    };
                    token_list.push(token);
                }
                else{
                    let error=lexer_error::LexerError{
                        kind: lexer_error::ErrorType::InvalidIndentifier,
                        line: _line
                    };
                    return Err(error);
                }
        }
    }
    let token=tokens::Token{
        kind: tokens::TokenType::EOC,
        line: _line,
        value: String::from(""),
    };
    token_list.push(token);
    return Ok(token_list);
}
fn is_special_caracter(caracter: char,list: &mut Vec<tokens::Token>,_line: usize)->bool{
    let token_kind=match caracter{
        '('=>tokens::TokenType::OPA,
        ')'=>tokens::TokenType::CPA,
        '{'=>tokens::TokenType::OCB,
        '}'=>tokens::TokenType::CCB,
        '['=>tokens::TokenType::OBR,
        ']'=>tokens::TokenType::CBR,
        ';'=>tokens::TokenType::EOL,
        ':'=>tokens::TokenType::COL,
        _=>tokens::TokenType::Void,
    };
    if token_kind==tokens::TokenType::Void  {
        return false;
    }
    let token = tokens::Token {
        kind: token_kind,
        line: _line,                    
        value: String::from(""),
    };
    list.push(token);
    return true;
}
fn is_useless(car:&char,lin:&mut usize)->bool{
    match *car{
        ' '|'\r'=>{
            return true;
        },
        '\t'=>{
            return true;
        },
        '\n'=>{
            *lin+=1;
            return true;
        }
        _=>return false,
    }
}
fn is_comentery(car: char)->bool{
    return car=='#';
}
fn is_number(list: &mut Vec<tokens::Token>,text: &String,_line: usize)->bool{
     match text.parse::<f64>() {
        Ok(_) => {
            let token = tokens::Token {
            kind: tokens::TokenType::Num,
            line: _line,                    
            value: text.clone(),
            };
            list.push(token);
            return true;
        },
        Err(_) => return false,
    };
}
fn is_keyword(list: &mut Vec<tokens::Token>,text: &String,_line: usize)->bool{
    let token_kind:tokens::TokenType;
    match text.as_str(){
        "pop"=>token_kind=tokens::TokenType::Pop,
        "push"=>token_kind=tokens::TokenType::Push,
        "print"=>token_kind=tokens::TokenType::Print,
        "if"=>token_kind=tokens::TokenType::If,
        "else"=>token_kind=tokens::TokenType::Else,
        "while"=>token_kind=tokens::TokenType::While,
        "var"=>token_kind=tokens::TokenType::Var,
        "arr"=>token_kind=tokens::TokenType::Arr,
        "read"=>token_kind=tokens::TokenType::Read,
        "add"=>token_kind=tokens::TokenType::Add,
        "sub"=>token_kind=tokens::TokenType::Sub,
        "mult"=>token_kind=tokens::TokenType::Mult,
        "div"=>token_kind=tokens::TokenType::Div,
        "mod"=>token_kind=tokens::TokenType::Mod,
        "equal"=>token_kind=tokens::TokenType::Equ,
        "bigger"=>token_kind=tokens::TokenType::Big,
        "lower"=>token_kind=tokens::TokenType::Low,
        "biggerEqual"=>token_kind=tokens::TokenType::BigEqu,
        "lowerEqual"=>token_kind=tokens::TokenType::LowEqu,
        "save"=>token_kind=tokens::TokenType::Save,
        "and"=>token_kind=tokens::TokenType::And,
        "or"=>token_kind=tokens::TokenType::Or,
        "not"=>token_kind=tokens::TokenType::Not,
        "True"=>token_kind=tokens::TokenType::True,
        "False"=>token_kind=tokens::TokenType::False,
        "Null"=>token_kind=tokens::TokenType::Null,
        "num"=>token_kind=tokens::TokenType::NumType,
        "bool"=>token_kind=tokens::TokenType::BoolType,
        "str"=>token_kind=tokens::TokenType::StrType,
        _=> return false,
    }
    let token=tokens::Token{
        kind: token_kind,
        line: _line,
        value: text.clone(),
    };
    list.push(token);
    return true;
}
fn is_invalid_caracter(caracter: char)->bool{
    match caracter{
        c if c.is_alphanumeric()||is_comentery(c)||is_usefull_special_caracter(c) => false,
        _=>true,
    }
}
fn is_usefull_special_caracter(caracter: char)->bool{
    return caracter=='('||caracter==')'||caracter=='{'
    ||caracter=='}'||caracter=='['||caracter==']'||caracter==';'
    ||caracter==' '||caracter=='\n'||caracter=='\r'
    ||caracter=='\t'||caracter=='\"'||caracter==':'
}
fn contains_invalid_caracter(ide: &String)->bool{
    for c in ide.chars(){
        if is_invalid_caracter(c){
            return true;
        }
    }
    false
}