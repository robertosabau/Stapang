pub struct Token{
    pub kind: TokenType,
    pub line: usize,
    pub value: String,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType{
    Void=0,
    //Keyworlds instructions
    Pop, //Take out first element of the Stack
    Push, //Put an element on the of the Stack
    Print, //Print the first element of the Stack
    If, //Conditional
    While, //Loop
    Var, //Declaration variable
    Arr, //Declaration array
    Save, //Modify save mode
    Read, //Read keyworld
    //KeyWorlds no instructions
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
    Ide, //Variable
    Num, //Number
    Str, //String
    //Especial Caracters
    OPA, //Open Parentesis
    CPA, //Close Parentesis
    OCB, //Open Curvy Bracket
    CCB, //Close Curvy Bracket
    OBR, //Open Bracket
    CBR, //Close Bracket
    EOL, // End of Line
    EOC, //End of Code
}