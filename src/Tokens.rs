#[derive(Debug, Clone,PartialEq)]
pub struct Token{
    pub kind: TokenType,
    pub line: usize,
    pub value: String,
}
#[derive(Clone,Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType{
    Void=0,
    //Keyworlds instructions
    Pop, //Take out first element of the Stack
    Push, //Put an element on the of the Stack
    Print, //Print the first element of the Stack
    If, //Conditional
    Else, //An else situation
    While, //Loop
    Var, //Declaration variable
    Arr, //Declaration array
    Safe, //Modify save mode
    Read, //Read keyworld
    //Keyworld no instructions
    True, //Boolean true
    False, //Boolean false
    Null, //No value
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
    Not, // Not
    And, //AND
    Or, //OR
    //Values
    Ide, //Variable
    Num, //Number
    Str, //String
    //Types
    NumType, //Type for numbers
    BoolType, //Type for boolean values
    StrType, //Type for strings
    AnyType, //Type to have every value posible
    //Especial Caracters
    OPA, //Open Parentesis
    CPA, //Close Parentesis
    OCB, //Open Curvy Bracket
    CCB, //Close Curvy Bracket
    OBR, //Open Bracket
    CBR, //Close Bracket
    COL, //Colon
    EOL, // End of Line
}