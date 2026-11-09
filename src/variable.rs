use crate::value;
pub struct Variable{
    kind: VariableType,
    value: value::Value,
}
#[derive(PartialEq,Clone,Debug)]
pub enum VariableType{
    Text,
    Number,
    Boolean,
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
        if self.is_compatible(value.kind.clone()){
            self.value=value;
            return true;
        }
        else{
            return false;
        }
    }
    //Function to get the value
    pub fn get_value(&self)->value::Value{
        self.value.clone()
    }
    //Function to check if is compatible the value with the variable
    fn is_compatible(&self, kind:value::ValueType)->bool{
        //Case of null, perfect
        if kind==value::ValueType::Null{
            return true;
        }
        //Cases of every type
        match self.kind{
            VariableType::Text => return matches!(kind, value::ValueType::Text(_)),
            VariableType::Number => return matches!(kind, value::ValueType::Number(_)),
            VariableType::Boolean => return matches!(kind, value::ValueType::Boolean(_)),
            VariableType::Any => return true,
        }
    }
}
impl VariableType{
    pub fn valid_to_read(&self)->bool{
        matches!(self,VariableType::Text)||matches!(self,VariableType::Number)
    }
}