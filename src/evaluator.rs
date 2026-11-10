use crate::{array, calculator, evaluator_error, node, value, value_source, variable::{self, Variable}};
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
    match evaluate_nodes(nodes,&mut variables,&mut arrays,&mut stack){
        Ok(boolean)=>{
            return Ok(boolean);
        }
        Err(error)=>{
            return Err(error);
        }
    }
}
//Function to send the node where it has to go
fn evaluate_nodes(nodes:Vec<node::Node>,variables:&mut HashMap<String, variable::Variable>,arrays:&mut HashMap<String,array::Array>,mut stack:&mut Vec<value::Value>)->Result<bool, evaluator_error::EvaluatorError>{
    //For to pass around every node
    for node in nodes{
        match node.kind{
            node::NodeType::AddNode|node::NodeType::AndNode|node::NodeType::GreaterEquNode|node::NodeType::GreaterNode|node::NodeType::DivNode
            |node::NodeType::EquNode|node::NodeType::LessEquNode|node::NodeType::SubNode|node::NodeType::MultNode|node::NodeType::ModNode
            |node::NodeType::LessNode|node::NodeType::OrNode|node::NodeType::NotNode =>{
                match operation_treatment(&mut stack, &node.kind){
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
                let array:array::Array=array::Array::new( kind,positions.parse::<usize>().unwrap());
                //Saving the variable at the hashmap for variable
                arrays.insert(name,array);
            }
            node::NodeType::VoidNode=>{
                return Ok(false);
            }
            node::NodeType::Popnode { ide } =>{
                match pop_traitment(variables, arrays, stack,ide){
                    Ok(_) =>{}
                    Err(error) =>return Err(evaluator_error::EvaluatorError{kind:error,line:node.tokens[0].line}),
                }
            }
            node::NodeType::Pushnode { .. } =>{
                match push_traitment(&mut stack, &node.kind, &variables, &arrays){
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
                if !print_traitment(&mut stack){
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
                            return Err(evaluator_error::EvaluatorError {
                                kind: evaluator_error::ErrorType::StackEmpty,
                                line: node.tokens[0].line 
                            });
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
                    match evaluate_nodes(instructions, variables, arrays, stack){
                        Ok(_) => continue,
                        Err(error) => return Err(error),
                    }
                }
                //Executing instruction in case of false
                else{
                    match evaluate_nodes(else_instructions, variables, arrays, stack){
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
                            return Err(evaluator_error::EvaluatorError {
                                kind: evaluator_error::ErrorType::StackEmpty,
                                line: node.tokens[0].line 
                            });
                        
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
                    match evaluate_nodes(instructions.clone(), variables, arrays, stack){
                        Ok(_) =>{}
                        Err(error) => return Err(error),
                    }
                    //Getting the value in the stack
                    value=match stack.pop(){
                        Some(value) => value,
                        None =>{
                            return Err(evaluator_error::EvaluatorError {
                                kind: evaluator_error::ErrorType::StackEmpty,
                                line: node.tokens[0].line 
                            });
                        } 
                    };
                }
            }
            //Treatment variable creation
            node::NodeType::VarNode { name,kind } => {
                //Creation of variable
                let var:variable::Variable=Variable::new(kind);
                //Saving the variable at the hashmap for variable
                variables.insert(name,var);
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
//Function to make the traitment of the print instruction
fn print_traitment(stack:&mut Vec<value::Value>)->bool{
    //Pop of the stack
    let value = match stack.pop() {
        Some(value) => value,
        None => {
            return false;
        }
    };
    let text=match value.kind{
        value::ValueType::Text(text) =>text,
        value::ValueType::Float(num) =>num.to_string(),
        value::ValueType::Int(num) =>num.to_string(),
        value::ValueType::Index(num) =>num.to_string(),
        value::ValueType::Boolean(bool) =>bool.to_string(),
        value::ValueType::Null => return false,
    };
    //Printing the value that was in ther first position of the stack
    print!("{}",text);
    //Flushing the output buffer
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    true
}
//Function to make the traitment of the push value
fn push_traitment(stack:&mut Vec<value::Value>,node:&node::NodeType,variables:&HashMap<String, variable::Variable>,arrays:&HashMap<String, array::Array>)->Result<bool,evaluator_error::ErrorType>{
    //Checking if we have entered with the correct node to get the atributes
    if let node::NodeType::Pushnode {value} = node {
        //Checking if a literal value
        if let value_source::ValueSource::Literal(value_to_push)=value{
            //Push the value directly
            stack.push(value_to_push.clone());
            //Code for ok status
            return Ok(true);
        }
        //Checking if is an identifier to get the identifier
        if let value_source::ValueSource::Identifier(ide)=value{
            match ide{
                //Case is a variable
                value_source::Identifier::Variable(name) =>{
                    //Checking if the varible was declared
                    if !variables.contains_key(name){
                        //Code for variable do not found
                        return Err(evaluator_error::ErrorType::VariableNotDefined { name:name.clone() });
                    }
                    //Push the value in the variable in the stack
                    if let Some(variable) = variables.get(name) {
                        if variable.get_value().is_a_null(){
                            return Err(evaluator_error::ErrorType::VariableNotInicializated { name:name.clone() });
                        }
                        stack.push(variable.get_value());
                    }
                    //Code for ok status
                    return Ok(true);
                }
                //Case is a array
                value_source::Identifier::Array { name, position } =>{
                    //Checking if the array was defined
                    if !arrays.contains_key(name){
                        //Code for array do not found
                        return Err(evaluator_error::ErrorType::ArrayNotDefined { name:name.clone() });
                    }
                    //Variable for the usize
                    let mut index:usize=0;
                    //Getting the position
                    match &**position{
                        value_source::ValueSource::Literal(val) =>{
                            if let value::ValueType::Index(num)=val.kind{
                                index=num as usize;
                            }
                        }
                        value_source::ValueSource::Identifier(identifier) =>{},
                    }
                    //Push the value in the variable if exists
                    if let Some(array) = arrays.get(name) {
                        match array.get_value(index.clone()){
                            Ok(value)=>{
                                if value.is_a_null(){
                                    return Err(evaluator_error::ErrorType::ArrayNotInicializated { name:name.clone(),position:index});
                                }
                                stack.push(value);
                                return Ok(true);
                            }
                            Err(_)=>{
                                //Out of bound
                                return Err(evaluator_error::ErrorType::OutOfBound);
                            }
                        }
                    }
                    else{
                        return Err(evaluator_error::ErrorType::InternarError);
                    }
                }
            }
        }
        else{
            Err(evaluator_error::ErrorType::InternarError)
        }
        //Continue in case there is only a variable, checking if the variable is defined
    }
    //return error (It's not supossed to happend, but just in case)
    else{
        //Code for internar error
        return Err(evaluator_error::ErrorType::InternarError);
    }
}
//Function to treat an operation
fn operation_treatment(stack:&mut Vec<value::Value>,node:&node::NodeType)->Result<bool,evaluator_error::ErrorType>{
    let return_value;
    let val1 = match stack.pop() {
        Some(value) => value,
        None => {
            return Err(evaluator_error::ErrorType::StackEmpty);
        }
    };
    if *node!=node::NodeType::NotNode{
        let val2 = match stack.pop() {
            Some(value) => value,
            None => {
                //Code for error of not element in stack
                return Err(evaluator_error::ErrorType::StackEmpty);
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
                match calculator::divide(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    Err(type_error) =>{
                        return Err(type_error);
                    }
                }
            }
            //Module
            node::NodeType::ModNode=>{
                match calculator::module(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(type_error) =>{
                        return Err(type_error);
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
            node::NodeType::GreaterNode=>{
                match calculator::greater(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //lower
            node::NodeType::LessNode=>{
                match calculator::less(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    Err(_) =>{
                            return Err(evaluator_error::ErrorType::InvalidType);
                    }
                }
            }
            //bigger or equal
            node::NodeType::GreaterEquNode=>{
                match calculator::greater_equal(&val1,&val2){
                    Ok(value) =>{
                        return_value=value;
                    }
                    //Returning code for invalid type
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidType),
                }
            }
            //lower or equal
            node::NodeType::LessEquNode=>{
                match calculator::less_equal(&val1,&val2){
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
fn treatment_read(stack:&mut Vec<value::Value>,format:&variable::VariableType)->Result<bool,evaluator_error::ErrorType>{
    let val;
    if !(format.valid_to_read()){
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
    if *format==variable::VariableType::Float{
        match input.parse::<f64>(){
            Ok(number) =>{
                val=value::Value::set_as_float(number);
            }
            Err(_) =>{
                return Err(evaluator_error::ErrorType::CannotConvertToNumber);
            }
        }
    }
    else{
        val=value::Value::set_as_text(input);
    }
    stack.push(val);
    Ok(true)
}
fn pop_traitment(variables:&mut HashMap<String, variable::Variable>,arrays:&mut HashMap<String, array::Array>,stack:&mut Vec<value::Value>,ide:value_source::Identifier)->Result<bool, evaluator_error::ErrorType>{
    let val = match stack.pop() {
        Some(value) => value,
        None => {
                return Err(evaluator_error::ErrorType::StackEmpty);
        }
    };
    match ide{
        value_source::Identifier::Variable(name) =>{
            if !variables.contains_key(&name){
                return Err(evaluator_error::ErrorType::VariableNotDefined { name:name.clone() });
            }
            let variable:&mut variable::Variable=variables.get_mut(&name).unwrap();
                if !variable.set_value(val){
                    return Err(evaluator_error::ErrorType::InvalidValue);
            }
            return Ok(true);
        }
        value_source::Identifier::Array { name, position } =>{
            //Checking if the array was defined
                if !arrays.contains_key(&name){
                    //Code for array do not found
                    return Err(evaluator_error::ErrorType::ArrayNotDefined { name:name.clone() });
                }
                //Variable for the usize
                let mut index:usize=0;
                //Getting the position
                match &*position{
                    value_source::ValueSource::Literal(val) =>{
                        if let value::ValueType::Index(num)=val.kind{
                            index=num as usize;
                        }
                    }
                    value_source::ValueSource::Identifier(identifier) =>{},
                }
                let array:&mut array::Array=arrays.get_mut(&name).unwrap();
                match array.set_value(val, index){
                    Ok(_) =>return Ok(true),
                    Err(error) =>return Err(error),
                }
            }
    }
}