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
        match self.is_compatible(value.clone()){
            Ok(val) =>{
                self.values[position]=val;
                return Ok(true);
            }

            Err(err) =>return Err(err),
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
    fn is_compatible(&self, val:value::Value)->Result<value::Value,evaluator_error::ErrorType>{
        //Case of null, perfect
        if val.kind==value::ValueType::Null{
            return Ok(val);
        }
        //Cases of every type
        match (self.kind.clone(),val.kind){
            //Case string value and string variable
            (variable::VariableType::Text,value::ValueType::Text(val))=>{
                return Ok(value::Value::set_as_text(val));
            }
            //Case boolean value and boolean variable
            (variable::VariableType::Boolean,value::ValueType::Boolean(val))=>{
                return Ok(value::Value::set_as_bool(val));
            }
            //Case uint value and uint variable
            (variable::VariableType::UInt,value::ValueType::UInt(val))=>{
                return Ok(value::Value::set_as_uint(val));
            }
            //Case int value and uint variable
            (variable::VariableType::UInt,value::ValueType::Int(val))=>{
                match usize::try_from(val){
                    Ok(num) =>return Ok(value::Value::set_as_uint(num)),
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidValue),
                }
            }
            //Case float value and uint variable
            (variable::VariableType::UInt,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_uint(val as usize));
            }
            //Case int value and int variable
            (variable::VariableType::Int,value::ValueType::Int(val))=>{
                return Ok(value::Value::set_as_int(val));
            }
            //Case uint value and int variable
            (variable::VariableType::Int,value::ValueType::UInt(val))=>{
                match i64::try_from(val){
                    Ok(num) =>return Ok(value::Value::set_as_int(num)),
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidValue),
                }
            }
            //Case float value and int variable
            (variable::VariableType::Int,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_int(val as i64));
            }
            //Case float value and float variable
            (variable::VariableType::Float,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_float(val));
            }
            //Case uint value and float variable
            (variable::VariableType::Float,value::ValueType::UInt(val))=>{
                return Ok(value::Value::set_as_float(val as f64));
            }
            //Case int value and float variable
            (variable::VariableType::Float,value::ValueType::Int(val))=>{
                return Ok(value::Value::set_as_float(val as f64));
            }
            _=>{
                return Err(evaluator_error::ErrorType::InvalidValue);
            }
        }
    }
}