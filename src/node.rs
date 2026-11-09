use crate::tokens;
use crate::value_source;
use crate::variable;

#[derive(PartialEq,Clone,Debug)]
pub struct Node{
    pub kind: NodeType,
    pub tokens: Vec<tokens::Token>,
}
#[derive(PartialEq,Clone,Debug)]
pub enum NodeType{
    VoidNode, //Node for the case their is not else instruction in the if node
    Popnode {ide:value_source::Identifier}, //Node for Pop instruction
    Pushnode {value:value_source::ValueSource}, //Node for push instruction
    PrintNode, //Node for print instruction
    IfNode{instructions:Vec<Node>,else_instructions:Vec<Node>}, //Node for if instruction
    WhileNode{instructions:Vec<Node>}, //Node for while instruction
    VarNode {name: String,kind:variable::VariableType}, //Node for var instruction
    ArrNode {name: String,positions:String,kind:variable::VariableType},//Node for arr instruction
    ReadNode {format:variable::VariableType}, //Node for read instruction
    AddNode, //Node for add instruction
    SubNode, //Node for sub instruction
    MultNode, //Node for mult instruction
    DivNode, //Node for div instruction
    ModNode, //Node for mod instruction
    EquNode, //Node for equal instruction
    GreaterNode, //Node for bigger instruction
    LessNode, //Node for lower instruction
    GreaterEquNode, //Node for biggerEqual instruction
    LessEquNode, //Node for lowerEqual instruction
    NotNode, //Node for not instruction
    AndNode, //Node for and instruction
    OrNode, //Node for or instruction
}