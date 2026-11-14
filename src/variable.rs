use crate::{evaluator_error, value};
pub struct Variable{
    kind: VariableType,
    value: value::Value,
}
#[derive(PartialEq,Clone,Debug)]
pub enum VariableType{
    Text,
    Float,
    Boolean,
    Int,
    UInt,
    Any,
}
impl Variable{
    //Constructor of a Variable
    pub fn new(kind:VariableType)->Self{
        Self {
            kind,
            value: value::Value::set_as_null(),
        }
    }
    //Function to set the value in a save way
    pub fn set_value(&mut self,value:value::Value)->bool{
        match self.is_compatible(value.clone()){
            Ok(val) =>{
                self.value=val;
                return true;
            },
            Err(_) => false,
        }
    }
    //Function to get the value
    pub fn get_value(&self)->value::Value{
        self.value.clone()
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
            (VariableType::Text,value::ValueType::Text(val))=>{
                return Ok(value::Value::set_as_text(val));
            }
            //Case boolean value and boolean variable
            (VariableType::Boolean,value::ValueType::Boolean(val))=>{
                return Ok(value::Value::set_as_bool(val));
            }
            //Case uint value and uint variable
            (VariableType::UInt,value::ValueType::UInt(val))=>{
                return Ok(value::Value::set_as_uint(val));
            }
            //Case int value and uint variable
            (VariableType::UInt,value::ValueType::Int(val))=>{
                match usize::try_from(val){
                    Ok(num) =>return Ok(value::Value::set_as_uint(num)),
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidValue),
                }
            }
            //Case float value and uint variable
            (VariableType::UInt,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_uint(val as usize));
            }
            //Case int value and int variable
            (VariableType::Int,value::ValueType::Int(val))=>{
                return Ok(value::Value::set_as_int(val));
            }
            //Case uint value and int variable
            (VariableType::Int,value::ValueType::UInt(val))=>{
                match i64::try_from(val){
                    Ok(num) =>return Ok(value::Value::set_as_int(num)),
                    Err(_) => return Err(evaluator_error::ErrorType::InvalidValue),
                }
            }
            //Case float value and int variable
            (VariableType::Int,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_int(val as i64));
            }
            //Case float value and float variable
            (VariableType::Float,value::ValueType::Float(val))=>{
                return Ok(value::Value::set_as_float(val));
            }
            //Case uint value and float variable
            (VariableType::Float,value::ValueType::UInt(val))=>{
                return Ok(value::Value::set_as_float(val as f64));
            }
            //Case int value and float variable
            (VariableType::Float,value::ValueType::Int(val))=>{
                return Ok(value::Value::set_as_float(val as f64));
            }
            _=>{
                return Err(evaluator_error::ErrorType::InvalidValue);
            }
        }
    }
}
impl VariableType{
    pub fn valid_to_read(&self)->bool{
        matches!(self,VariableType::Text)||matches!(self,VariableType::Float)||matches!(self,VariableType::Int)||matches!(self,VariableType::UInt)
    }
}