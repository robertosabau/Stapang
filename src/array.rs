use crate::{ evaluator_error::{self}, value};
use crate::variable;
pub struct Array{
    kind: variable::VariableType,
    values: Vec<value::Value>,
    positions:usize,
}
impl Array {
    //Constructor of a Array
    pub fn new(kind:variable::VariableType,position:usize)->Self{
        Self { 
            kind,
            values:vec![value::Value::set_as_null();
            position],
            positions:position,
        }
    }//Function to set the value in a save way
    pub fn set_value(&mut self,value:value::Value,position:usize)->Result<bool,evaluator_error::ErrorType>{
        if position>=self.positions{
            return Err(evaluator_error::ErrorType::OutOfBound);
        }
        if self.is_compatible(value.kind.clone()){
            self.values[position]=value;
            return Ok(true);
        }
        else{
            return Err(evaluator_error::ErrorType::InvalidValue);
        }
    }
    //Function to get the value
    pub fn get_value(&self,position:usize)->Result<value::Value,u8>{
        //Checking if the position is out of bounds
        if position>=self.positions{
            return Err(0);
        }
        Ok(self.values[position].clone())
    }
    //Function to check if is compatible the value with the variable
    fn is_compatible(&self, kind:value::ValueType)->bool{
        //Case of null, perfect
        if kind==value::ValueType::Null{
            return true;
        }
        //Cases of every type
        match self.kind{
            variable::VariableType::Text => return matches!(kind, value::ValueType::Text(_)),
            variable::VariableType::Float => return matches!(kind, value::ValueType::Float(_))||matches!(kind, value::ValueType::Int(_))||matches!(kind, value::ValueType::Index(_)),
            variable::VariableType::Boolean => return matches!(kind, value::ValueType::Boolean(_)),
            variable::VariableType::Any => return true,
            variable::VariableType::Int => return matches!(kind, value::ValueType::Int(_))||matches!(kind, value::ValueType::Index(_)),
            variable::VariableType::Index =>matches!(kind, value::ValueType::Index(_)),
        }
    }
}