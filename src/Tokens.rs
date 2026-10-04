pub struct Token{
    pub kind: TokenType,
    pub line: usize,
    pub colum: usize
}
pub enum TokenType{
    Void=0,
    //Keyworlds
    Pop,
    Push,
    Print,
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    //Values
    Var, //Variable
    Num, //Number
    Car, //Caracter
    Str //String
}