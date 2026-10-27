use crate::{evaluator_error, node, value, variable};
use std::collections::HashMap;

//Function to start the evaluation
pub fn evaluation(nodes:Vec<node::Node>)->Result<bool, evaluator_error::EvaluatorError>{
    //Checking if there is an save instruction
    if nodes.iter().any(|n| matches!(n.kind, node::NodeType::SaveNode { .. })) {
        //Warning to advice that during the execution of the program the save mode is changed
        println!("WARNING: Save mode will change during execution. Results may be unpredictable and errors harder to trace.");
    }
    match evaluate_node(nodes){
        Ok(boolean)=>{
            return Ok(boolean);
        }
        Err(error)=>{
            return Err(error);
        }
    }
}
//Function to send the node where it has to go
fn evaluate_node(nodes:Vec<node::Node>)->Result<bool, evaluator_error::EvaluatorError>{
    //HashMap for the variable
    let mut variables: HashMap<String, variable::Variable> = HashMap::new();
    //The stack of the system
    let mut stack:Vec<value::Value>=Vec::new();
    //Boolean for the save_mode
    let mut save_mode:bool=true;
    //For to pass around every node
    for node in nodes{
        match node.kind{
            node::NodeType::AddNode=>{

            }
            node::NodeType::AndNode=>{
                
            }
            node::NodeType::ArrNode { .. }=>{

            }
            node::NodeType::BigEquNode=>{
                
            }
            node::NodeType::BigNode=>{

            }
            node::NodeType::DivNode=>{
                
            }
            node::NodeType::EquNode=>{

            }
            node::NodeType::LowEquNode=>{
                
            }
            node::NodeType::VoidNode=>{

            }
            node::NodeType::Popnode { .. } =>{

            }
            node::NodeType::Pushnode { ..} =>{

            }
            node::NodeType::PrintNode =>{
                if !print_traitment(&mut stack, &save_mode){
                    let error=evaluator_error::EvaluatorError{
                        kind:evaluator_error::ErrorType::StackEmpty,
                        line:node.tokens[0].line,
                    };
                    return Err(error);
                }
            },
            node::NodeType::IfNode { .. } =>{

            }
            node::NodeType::WhileNode { .. } => {

            }
            node::NodeType::VarNode { .. } => {

            }
            node::NodeType::SaveNode { value } =>{
                if !save_traitment(&value, &mut save_mode){
                    let error=evaluator_error::EvaluatorError{
                        kind:evaluator_error::ErrorType::BooleanExpected,
                        line: node.tokens[0].line,
                    };
                    return Err(error);
                }
            }
            node::NodeType::ReadNode => {

            }
            node::NodeType::SubNode => {

            }
            node::NodeType::MultNode => {

            }
            node::NodeType::ModNode => {
                
            }
            node::NodeType::LowNode => {
                
            }
            node::NodeType::NotNode => {
                
            }
            node::NodeType::OrNode => {
                
            }
        }
    }
    Ok(true)
}
//Function to make the traitment of the save instruction
fn save_traitment(value:&value::Value,mode:&mut bool)->bool{
    //Cases for the value intruduced
    match value.value.as_str(){
        "True"=>{
            *mode=true;
        }
        "False"=>{
            *mode=false;
        }
        _=> return false
    }
    true
}
//Function to make the traitment of the print instruction
fn print_traitment(stack:&mut Vec<value::Value>,mode:&bool)->bool{
    //Pop of the stack
    let mut value=stack.pop();
    //Checking if the stack was empty and was returned a None
    match value{
        Some(_)=>{
        }
        None=>{
            //Checking if the save mode is on
            if *mode{
                return false;
            }
            //Getting an "error" value for the save mode off
            value=Some(value::Value{kind:value::ValueType::Null,value:String::from("Null")});
        }
    }
    //Printing the value that was in ther first position of the stack
    print!("{}",value.unwrap().value.as_str());
    true
}