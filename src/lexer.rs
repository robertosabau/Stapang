use crate::tokens;


pub fn tokenize(code: &str) -> Vec<tokens::Token>{
    let token_list: Vec<tokens::Token>=Vec::new();
    let caracteres: Vec<char> = code.chars().collect();
    let mut index=0;
    let mut _columm=1;
    let mut _line=1;
    let mut caracter;
    while index<caracteres.len(){
        caracter=caracteres[index];
        while is_useless(&caracter, &mut _line, &mut _columm){
            index+=1;
            caracter=caracteres[index];
        }
        if is_special_caracter(caracter, token_list,_line,_columm){
            _columm+=1;
            index += 1;
            continue;
        }
    }
    return token_list;
}
fn is_special_caracter(caracter: char,list: Vec<tokens::Token>,_line: usize,_columm: usize)->bool{
    let token_kind=match caracter{
        '('=>tokens::TokenType::OPA,
        ')'=>tokens::TokenType::CPA,
        '{'=>tokens::TokenType::OCB,
        '}'=>tokens::TokenType::CCB,
        '['=>tokens::TokenType::OBR,
        ']'=>tokens::TokenType::CBR,
        ';'=>tokens::TokenType::EOL,
        _=>tokens::TokenType::Void,
    };
    if token_kind==tokens::TokenType::Void  {
        return false;
    }
    let token = tokens::Token {
    kind: token_kind,
    line: _line,
    colum: _columm,                     
    value: String::from("valor"), 
};
    return true;
}
fn is_useless(car:&char,lin:&mut usize,col:&mut usize)->bool{
    match *car{
        ' '|'\r'=>{
            *col+=1;
            return true;
        },
        '\t'=>{
            *col+=4;
            return true;
        },
        '\n'=>{
            *col=0;
            *lin+=1;
            return true;
        }
        _=>return false,
    }
}