use crate::{ evaluator_error::{self}, value};
pub struct Array{
    kind: value::ValueType,
    values: Vec<value::Value>,
    positions:usize,
}
impl Array {
    //Constructor of a Array
    pub fn new(kind:value::ValueType,position:usize)->Self{
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
        match value.kind{
            value::ValueType::Null=>{
                self.values[position]=value;
                return Ok(true);
            }
            _=>{
                if self.kind==value::ValueType::Any||self.kind==value.kind{
                    self.values[position]=value;
                    return Ok(true);
                }
                Err(evaluator_error::ErrorType::InvalidValue)
            }
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
}