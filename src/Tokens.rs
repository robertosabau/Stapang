pub struct Token{
    pub kind: TokenType,
    pub line: usize,
    pub colum: usize,
    pub value: String
}
pub enum TokenType{
    Void=0,
    //Keyworlds
    Pop, //Take out first element of the Stack
    Push, //Put an element on the of the Stack
    Print, //Print the first element of the Stack
    If, //Conditional
    While, //Loop
    //Operators
    Add, // +
    Sub, // -
    Mult, // *
    Div, // /
    Mod, // %
    Equ, // =
    Big, // >
    Low, // <
    BigEqu, //>=
    LowEqu, //<=
    //Values
    Var, //Variable
    Num, //Number
    Car, //Caracter
    Str, //String
    //Especial Caracters
    OCB, //Open Curvy Bracket
    CCB, //Close Curvy Bracket
    EOL, // End of Line
    EOC, //End of Code
}