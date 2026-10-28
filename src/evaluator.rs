use crate::{array, evaluator_error::{self, ErrorType::InternarError, EvaluatorError}, node, value, variable};
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
    //HashMap for the arrays
    let mut arrays:HashMap<String,array::Array>=HashMap::new();
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
                return Ok(false);
            }
            node::NodeType::Popnode { .. } =>{

            }
            node::NodeType::Pushnode {ref name, ..} =>{
                match push_traitment(&mut stack, &save_mode, &node.kind, &variables, &arrays){
                    0=>{
                    },
                    1=>{
                        let error=evaluator_error::EvaluatorError{
                            kind:evaluator_error::ErrorType::InternarError,
                            line:node.tokens[0].line,
                        };
                        return Err(error);
                    }
                    2=>{
                        let error=evaluator_error::EvaluatorError{
                            kind:evaluator_error::ErrorType::VariableNotDefined{name:name.clone()},
                            line:node.tokens[0].line,
                        };
                        return Err(error);
                    }
                    3=>{
                        let error=evaluator_error::EvaluatorError{
                            kind:evaluator_error::ErrorType::ArrayNotDefined {name:name.clone()},
                            line:node.tokens[0].line,
                        };
                        return Err(error);
                    }
                    4=>{
                        let error=evaluator_error::EvaluatorError{
                            kind:evaluator_error::ErrorType::OutOfBound,
                            line:node.tokens[0].line,
                        };
                        return Err(error);
                    }
                    _=>{
                    }
                }
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
    let value = match stack.pop() {
        Some(value) => value,
        None => {
            // Checking if the safe mode is on
            if *mode {
                return false;
            }
            // Getting a "Null" value for the safe mode off
            value::Value {
                kind: value::ValueType::Null,
                value: String::from("Null"),
            }
        }
    };
    //Printing the value that was in ther first position of the stack
    print!("{}",value.value.as_str());
    true
}
//Function to make the traitment of the push value
fn push_traitment(stack:&mut Vec<value::Value>,mode:&bool,node:&node::NodeType,variables:&HashMap<String, variable::Variable>,arrays:&HashMap<String, array::Array>)->u8{
    //Checking if we have entered with the correct node to get the atributes
    if let node::NodeType::Pushnode {array,name,position,value,} = node {
        //Checking if a value, which is assigned to name de String "Null"
        if *name==String::from("Null"){
            //Push the value directly
            stack.push(value.clone());
            //Code for ok status
            return 0;
            
        }
        //Checking if is an Array
        if *array==true{
            //Checking if the array was defined
            if !arrays.contains_key(name){
                //Checking if the save mode is on
                if *mode{
                    //Code for array do not found
                    return 3;
                }
                //Pushing a null value
                stack.push(value::Value { kind: value::ValueType::Null, value:String::from("Null")});
                //Code for ok status
                return 0;
            }
            if let Some(array) = arrays.get(name) {
                match array.get_value(position.unwrap()){
                    Ok(value)=>{
                        stack.push(value);
                        //Code for ok status
                        return 0;
                    }
                    Err(_code)=>{
                        //Code for out of bounds error
                        return 4;
                    }
                }
            }

        }
        //Continue in case there is only a variable, checking if the variable is defined
        if !variables.contains_key(name){
            //Checking if the save mode is on
            if *mode{
                //Code for variable do not found
                return 2;
            }
            stack.push(value::Value { kind: value::ValueType::Null, value:String::from("Null")});
            //Code for ok status
            return 0;
        }
        //Push the value in the variable in the stack
        if let Some(variable) = variables.get(name) {
            stack.push(variable.get_value());
        }
        //Code for ok status
        return 0;
    }
    //return error (It's not supossed to happend, but just in case)
    else{
        //Code for internar error
        return 1;
    }
}
//Function to treat an operation
pub fn operation_treatment(stack:&mut Vec<value::Value>,mode:&bool)->u8{
    
    0
}