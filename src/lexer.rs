use crate::tokens;

pub fn tokenize(code: String) -> Vec<tokens::Token>{
    let mut token_list: Vec<tokens::Token>=Vec::new();
    let caracteres: Vec<char> = code.chars().collect();
    let mut index=0;
    let mut _line=1;
    let mut caracter;
    while index<caracteres.len(){
        caracter=caracteres[index];
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
            caracter=caracteres[index];
            let mut name:String=String::from("");
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
                continue;
            }

        }
        else{
            let mut name:String=String::from("");
            while index<caracteres.len()&&!is_useless(&caracter, &mut _line){
                name.push(caracter);
                index+=1;
                if index<caracteres.len(){
                    caracter=caracteres[index];
                }
                if is_number(&mut token_list,&name, _line){
                    index+=1;
                    continue;
                }
                
            }

        }
    }
    return token_list;
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
fn is_number(list: &mut Vec<tokens::Token>,text: &str,_line: usize)->bool{
     match text.parse::<f64>() {
        Ok(_) => {
            let token = tokens::Token {
            kind: tokens::TokenType::Num,
            line: _line,                    
            value: String::from(text),
            };
            list.push(token);
            return true;
        },
        Err(_) => return false,
    };
}