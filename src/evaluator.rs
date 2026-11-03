use crate::{array, calculator, evaluator_error, node, value, variable::{self, Variable}};
use std::collections::HashMap;
use std::io;

//Function to start the evaluation
pub fn evaluation(nodes:Vec<node::Node>)->Result<bool, evaluator_error::EvaluatorError>{
    //HashMap for the variable
    let mut variables: HashMap<String, variable::Variable> = HashMap::new();
    //HashMap for the arrays
    let mut arrays:HashMap<String,array::Array>=HashMap::new();
    //The stack of the system
    let mut stack:Vec<value::Value>=Vec::new();
    //Boolean for the save_mode
    let mut save_mode:bool=true;
    //Checking if there is an save instruction
    if nodes.iter().any(|n| matches!(n.kind, node::NodeType::SaveNode { .. })) {
        //Warning to advice that during the execution of the program the save mode is changed
        println!("WARNING: Save mode will change during execution. Results may be unpredictable and errors harder to trace.");
    }
    match evaluate_nodes(nodes,&mut variables,&mut arrays,&mut stack,&mut save_mode){
        Ok(boolean)=>{
            return Ok(boolean);
        }
        Err(error)=>{
            return Err(error);
        }
    }
}
//Function to send the node where it has to go
fn evaluate_nodes(nodes:Vec<node::Node>,variables:&mut HashMap<String, variable::Variable>,arrays:&mut HashMap<String,array::Array>,mut stack:&mut Vec<value::Value>,save_mode:&mut bool)->Result<bool, evaluator_error::EvaluatorError>{
    //For to pass around every node
    for node in nodes{
        match node.kind{
            node::NodeType::AddNode|node::NodeType::AndNode|node::NodeType::BigEquNode|node::NodeType::BigNode|node::NodeType::DivNode
            |node::NodeType::EquNode|node::NodeType::LowEquNode|node::NodeType::SubNode|node::NodeType::MultNode|node::NodeType::ModNode
            |node::NodeType::LowNode|node::NodeType::OrNode|node::NodeType::NotNode =>{
                match operation_treatment(&mut stack, &save_mode, &node.kind){
                    Ok(_) => continue,
                    Err(error) =>{
                        let error=evaluator_error::EvaluatorError{
                            kind:error,
                            line:node.tokens[0].line,
                        };
                        return Err(error);
                    }
                }

            }
            node::NodeType::ArrNode {name,positions,kind }=>{
                //Creation of variable
                let array:array::Array=array::Array::new(name.clone(), kind,positions.parse::<usize>().unwrap());
                //Saving the variable at the hashmap for variable
                arrays.insert(name,array);
            }
            node::NodeType::VoidNode=>{
                return Ok(false);
            }
            node::NodeType::Popnode { .. } =>{

            }
            node::NodeType::Pushnode { .. } =>{
                match push_traitment(&mut stack, &save_mode, &node.kind, &variables, &arrays){
                    Ok(_) => continue,
                    Err(error) => {
                        let err=evaluator_error::EvaluatorError{
                            kind:error,
                            line:node.tokens[0].line
                        };
                        return Err(err);
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
            node::NodeType::IfNode { instructions,else_instructions } =>{
                //Getting the value in the stack
                let value=match stack.pop(){
                    Some(value) => value,
                    None =>{
                        if *save_mode{
                            return Err(evaluator_error::EvaluatorError {
                                kind: evaluator_error::ErrorType::StackEmpty,
                                line: node.tokens[0].line 
                            });
                        }
                        value::Value::set_as_null()
                    } 
                };
                //Checking if the value is not a boolean
                if !(value.is_a_true()||value.is_a_false()){
                    return Err(evaluator_error::EvaluatorError{
                        kind: evaluator_error::ErrorType::BooleanExpected,
                        line: node.tokens[0].line,
                    })
                }
                //Executing instruction in case of true
                if value.is_a_true(){
                    match evaluate_nodes(instructions, variables, arrays, stack, save_mode){
                        Ok(_) => continue,
                        Err(error) => return Err(error),
                    }
                }
                //Executing instruction in case of false
                else{
                    match evaluate_nodes(else_instructions, variables, arrays, stack, save_mode){
                        Ok(_) => continue,
                        Err(error) => return Err(error),
                    }
                }
            }
            node::NodeType::WhileNode { instructions } => {
                //Getting the value in the stack
                let mut value=match stack.pop(){
                    Some(value) => value,
                    None =>{
                        if *save_mode{
                            return Err(evaluator_error::EvaluatorError {
                                kind: evaluator_error::ErrorType::StackEmpty,
                                line: node.tokens[0].line 
                            });
                        }
                        value::Value::set_as_null()
                    } 
                };
                //Checking if the value is not a boolean
                if !(value.is_a_true()||value.is_a_false()){
                    return Err(evaluator_error::EvaluatorError{
                        kind: evaluator_error::ErrorType::BooleanExpected,
                        line: node.tokens[0].line,
                    })
                }
                //Entering a while if the value was a boolean true
                while value.is_a_true(){
                    match evaluate_nodes(instructions.clone(), variables, arrays, stack, save_mode){
                        Ok(_) =>{}
                        Err(error) => return Err(error),
                    }
                    //Getting the value in the stack
                    value=match stack.pop(){
                        Some(value) => value,
                        None =>{
                            if *save_mode{
                                return Err(evaluator_error::EvaluatorError {
                                    kind: evaluator_error::ErrorType::StackEmpty,
                                    line: node.tokens[0].line 
                                });
                            }
                            value::Value::set_as_null()
                        } 
                    };
                }
            }
            //Treatment variable creation
            node::NodeType::VarNode { name,kind } => {
                //Creation of variable
                let var:variable::Variable=Variable::new(name.clone(), kind);
                //Saving the variable at the hashmap for variable
                variables.insert(name,var);
            }
            node::NodeType::SaveNode { value } =>{
                if !save_traitment(&value, save_mode){
                    let error=evaluator_error::EvaluatorError{
                        kind:evaluator_error::ErrorType::BooleanExpected,
                        line: node.tokens[0].line,
                    };
                    return Err(error);
                }
            }
            node::NodeType::ReadNode {format} => {
                match treatment_read(stack, &format){
                    Ok(_) =>{}
                    Err(error) => {
                        let err=evaluator_error::EvaluatorError{
                            kind:error,
                            line:node.tokens[0].line
                        };
                        return Err(err);
                    }
                }
            }
        }
    }
    Ok(true)
}
//Function to make the traitment of the save instruction
fn save_traitment(value:&value::Value,mode:&mut bool)->bool{
    //Cases for the value intruduced
    if value.is_a_true(){
        *mode=true;
    }
    else{
        *mode=true;
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
            value::Value::set_as_null()
        }
    };
    //Printing the value that was in ther first position of the stack
    print!("{}",value.value.as_str());
    //Flushing the output buffer
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    true
}
//Function to make the traitment of the push value
fn push_traitment(stack:&mut Vec<value::Value>,mode:&bool,node:&node::NodeType,variables:&HashMap<String, variable::Variable>,arrays:&HashMap<String, array::Array>)->Result<bool,evaluator_error::ErrorType>{
    //Checking if we have entered with the correct node to get the atributes
    if let node::NodeType::Pushnode {array,name,position,value,} = node {
        //Checking if a value, which is assigned to name de String "Null"
        if *name==String::from("Null"){
            //Push the value directly
            stack.push(value.clone());
            //Code for ok status
            return Ok(true);
            
        }
        //Checking if is an Array
        if *array==true{
            //Checking if the array was defined
            if !arrays.contains_key(name){
                //Checking if the save mode is on
                if *mode{
                    //Code for array do not found
                    return Err(evaluator_error::ErrorType::ArrayNotDefined { name:name.clone() });
                }
                //Pushing a null value
                stack.push(value::Value::set_as_null());
                //Code for ok status
                return Ok(true);
            }
            if let Some(array) = arrays.get(name) {
                match array.get_value(position.unwrap()){
                    Ok(value)=>{
                        stack.push(value);
                        //Code for ok status
                        return Ok(true);
                    }
                    Err(_code)=>{
                        //Code for out of bounds error
                        return Err(evaluator_error::ErrorType::OutOfBound);
                    }
                }
            }

        }
        //Continue in case there is only a variable, checking if the variable is defined
        if !variables.contains_key(name){
            //Checking if the save mode is on
            if *mode{
                //Code for variable do not found
                return Err(evaluator_error::ErrorType::VariableNotDefined { name:name.clone() });
            }
            stack.push(value::Value { kind: value::ValueType::Null, value:String::from("Null")});
            //Code for ok status
            return Ok(true);
        }
        //Push the value in the variable in the stack
        if let Some(variable) = variables.get(name) {
            stack.push(variable.get_value());
        }
        //Code for ok status
        return Ok(true);
    }
    //return error (It's not supossed to happend, but just in case)
    else{
        //Code for internar error
        return Err(evaluator_error::ErrorType::InternarError);
    }
}
//Function to treat an operation
fn operation_treatment(stack:&mut Vec<value::Value>,mode:&bool,node:&node::NodeType)->Result<bool,evaluator_error::ErrorType>{
    let return_value;
    let val1 = match stack.pop() {
        Some(value) => value,
        None => {
            // Checking if the safe mode is on
            if *mode {
                return Err(evaluator_error::ErrorType::InternarError);
            }
            // Getting a "Null" value for the safe mode off
            value::Value::set_as_null()
        }
    };
    if *node!=node::NodeType::NotNode{
        let val2 = match stack.pop() {
            Some(value) => value,
            None => {
                // Checking if the safe mode is on
                if *mode {
                    //Code for error of not element in stack
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                // Getting a "Null" value for the safe mode off
                value::Value::set_as_null()
            }
        };
        //Match to make the operation
        match node{
            //Addittion
            node::NodeType::AddNode=>{
                match calculator::addition(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //Substraction
            node::NodeType::SubNode=>{
                match calculator::subtraction(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //multiplication
            node::NodeType::MultNode=>{
                match calculator::multiply(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //Dividiton
            node::NodeType::DivNode=>{
                match calculator::divide(&val1,&val2,mode){
                    Ok(value) =>{
                        return_value=value;
                    }
                    Err(type_error) =>{
                        if type_error{
                            //code for invalid type
                            return Err(evaluator_error::ErrorType::DivisionByZero);
                        }
                        else{
                            //Code for matematical error
                            return Err(evaluator_error::ErrorType::InvalidType);
                        }
                    }
                }
            }
            //Module
            node::NodeType::ModNode=>{
                match calculator::module(&val1,&val2,mode){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(type_error) =>{
                        if type_error{
                            //code for invalid type
                            return Err(evaluator_error::ErrorType::DivisionByZero);
                        }
                        else{
                            //Code for matematical error
                            return Err(evaluator_error::ErrorType::InvalidType);
                        }
                    }
                }
            }
            //AND
            node::NodeType::AndNode=>{
                match calculator::and(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //OR
            node::NodeType::OrNode=>{
                match calculator::or(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //Equal
            node::NodeType::EquNode=>{
                match calculator::equals(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //Bigger
            node::NodeType::BigNode=>{
                match calculator::greater(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //lower
            node::NodeType::LowNode=>{
                match calculator::lower(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    Err(_) =>{
                            return Err(evaluator_error::ErrorType::InvalidType);
                    }
                }
            }
            //bigger or equal
            node::NodeType::BigEquNode=>{
                match calculator::greater_equal(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //lower or equal
            node::NodeType::LowEquNode=>{
                match calculator::lower_equal(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            _=>{
                //Code for internar error
                return Err(evaluator_error::ErrorType::InternarError);
            }
        }
    }
    else{
        match calculator::not(&val1){
            Ok(value) =>{
                return_value=value;
            }
            //Returning code for invalid type
            Err(_) => return Err(evaluator_error::ErrorType::InternarError),
        }
    }
    //Putting the value at the stack to have it
    stack.push(return_value);
    //Code for everything okey
    Ok(true)
}
fn treatment_read(stack:&mut Vec<value::Value>,format:&value::ValueType)->Result<bool,evaluator_error::ErrorType>{
    let mut val=value::Value{kind:format.clone(),value:String::from("")};
    if !(val.is_a_number()||val.is_a_text()){
        return Err(evaluator_error::ErrorType::InvalidType);
    }
    let mut input=String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => {}
        Ok(_) => {}
        Err(_) => {
            return Err(evaluator_error::ErrorType::ReadError);
        }
    }
    input=input.trim_end_matches(&['\n', '\r'][..]).to_string();
    if val.is_a_number(){
        match input.parse::<f64>(){
            Ok(_) =>{}
            Err(_) =>{
                return Err(evaluator_error::ErrorType::CannotConvertToNumber);
            }
        }
        val=value::Value::set_as_number(input);
    }
    else{
        val=value::Value::set_as_text(input);
    }
    stack.push(val);
    Ok(true)
}