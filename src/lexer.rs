use crate::tokens;
use crate::lexer_error;

pub fn tokenize(code: String) -> Result<Vec<tokens::Token>, lexer_error::LexerError> {
    let mut token_list = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut index = 0;
    let mut line = 1;

    while index < chars.len() {
        let c = chars[index];
        if c == '\n' { line += 1; index += 1; continue; }
        if c == ' ' || c == '\r' || c == '\t' { index += 1; continue; }
        if is_invalid_caracter(c) {
            return Err(lexer_error::LexerError {
                kind: lexer_error::ErrorType::InvalidCharacter, line });
        }
        if is_special_caracter(c, &mut token_list, line) { index += 1; continue; }
        if is_comentery(c) {
            while index < chars.len() && chars[index] != '\n' { index += 1; }
            continue;
        }
        if c == '"' {
            let start_line = line;
            index += 1;
            let mut name = String::new();
            while index < chars.len() && chars[index] != '"' {
                if chars[index] == '\\' {
                    index += 1;

                    if index >= chars.len() {
                        return Err(lexer_error::LexerError {
                            kind: lexer_error::ErrorType::UnclosedString,
                            line: start_line,
                        });
                    }

                    match chars[index] {
                        'n' => name.push('\n'),
                        't' => name.push('\t'),
                        '"' => name.push('"'),
                        '\\' => name.push('\\'),
                        other => {
                            name.push('\\');
                            name.push(other);
                        }
                    }
                    index+=1;
                }
                else{
                    if chars[index] == '\n' { line += 1; }
                    name.push(chars[index]);
                    index += 1;
                }
            }
            if index >= chars.len() {
                return Err(lexer_error::LexerError {
                    kind: lexer_error::ErrorType::UnclosedString, line: start_line });
            }
            index += 1; // cerrar comilla
            token_list.push(tokens::Token {
                kind: tokens::TokenType::Str, line: start_line, value: name });
            continue;
        }
        let mut name = String::new();
        while index < chars.len() && !is_delimiter(chars[index]) {
            name.push(chars[index]);
            index += 1;
        }
        if is_number(&mut token_list, &name, line) { continue; }
        if is_keyword(&mut token_list, &name, line) { continue; }
        if contains_invalid_caracter(&name) {
            return Err(lexer_error::LexerError {
                kind: lexer_error::ErrorType::InvalidIndentifier, line });
        }
        token_list.push(tokens::Token {
            kind: tokens::TokenType::Ide, line, value: name });
    }
    Ok(token_list)
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
        "greater"=>token_kind=tokens::TokenType::Greater,
        "less"=>token_kind=tokens::TokenType::Less,
        "greaterEqual"=>token_kind=tokens::TokenType::GreaterEqu,
        "lessEqual"=>token_kind=tokens::TokenType::LessEqu,
        "and"=>token_kind=tokens::TokenType::And,
        "or"=>token_kind=tokens::TokenType::Or,
        "not"=>token_kind=tokens::TokenType::Not,
        "true"=>token_kind=tokens::TokenType::True,
        "false"=>token_kind=tokens::TokenType::False,
        "null"=>token_kind=tokens::TokenType::Null,
        "index"=>token_kind=tokens::TokenType::IndexType,
        "int"=>token_kind=tokens::TokenType::IntType,
        "float"=>token_kind=tokens::TokenType::FloatType,
        "bool"=>token_kind=tokens::TokenType::BoolType,
        "str"=>token_kind=tokens::TokenType::StrType,
        "any"=>token_kind=tokens::TokenType::AnyType,
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
        c if c.is_alphanumeric()||is_comentery(c)||is_usefull_special_caracter(c)||c=='-'||c=='_' => false,
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
fn is_delimiter(c: char) -> bool {
    c.is_whitespace()
        || matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | ';' | ':' | '"' | '#')
}