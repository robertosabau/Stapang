use crate::tokens;

pub struct Node{
    kind: NodeType,
    tokens: Vec<tokens::Token>,
}

pub enum NodeType{
    Popnode, //Node for Pop instruction
    Pushnode,
    PrintNode,
    IfNode,
    WhileNode,
    VarNode,
    ArrNode,
    SaveNode,
    ReadNode,
}