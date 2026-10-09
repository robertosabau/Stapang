use crate::tokens;

pub struct Node{
    pub kind: NodeType,
    pub tokens: Vec<tokens::Token>,
    pub array: bool,
    pub position: usize,
}

pub enum NodeType{
    Popnode, //Node for Pop instruction
    Pushnode, //Node for push instruction
    PrintNode, //Node for print instruction
    IfNode, //Node for if instruction
    WhileNode, //Node for while instruction
    VarNode, //Node for var instruction
    ArrNode, //Node for arr instruction
    SaveNode, //Node for save instruction
    ReadNode, //Node for read instruction
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