use crate::tokens;
use crate::value;
use crate::value::ValueType;

#[derive(PartialEq,Clone,Debug)]
pub struct Node{
    pub kind: NodeType,
    pub tokens: Vec<tokens::Token>,
}
#[derive(PartialEq,Clone,Debug)]
pub enum NodeType{
    VoidNode, //Node for the case their is not else instruction in the if node
    Popnode {array:bool,position:usize,name: String}, //Node for Pop instruction
    Pushnode {array:bool,name:String,position:Option<usize>,value:value::Value}, //Node for push instruction
    PrintNode, //Node for print instruction
    IfNode{instructions:Vec<Node>,else_instructions:Vec<Node>}, //Node for if instruction
    WhileNode{instructions:Vec<Node>}, //Node for while instruction
    VarNode {name: String,kind:ValueType}, //Node for var instruction
    ArrNode {name: String,positions:String,kind:ValueType},//Node for arr instruction
    SafeNode {value: value::Value}, //Node for safe instruction
    ReadNode {format:value::ValueType}, //Node for read instruction
    AddNode, //Node for add instruction
    SubNode, //Node for sub instruction
    MultNode, //Node for mult instruction
    DivNode, //Node for div instruction
    ModNode, //Node for mod instruction
    EquNode, //Node for equal instruction
    BigNode, //Node for bigger instruction
    LowNode, //Node for lower instruction
    BigEquNode, //Node for biggerEqual instruction
    LowEquNode, //Node for lowerEqual instruction
    NotNode, //Node for not instruction
    AndNode, //Node for and instruction
    OrNode, //Node for or instruction
}