use crate::value;
pub struct Variable{
    identifier: String,
    kind: value::ValueType,
    value: value::Value,
}
impl Variable{
    //Constructor of a Variable
    fn new(name:String,kind:value::ValueType)->Self{
        Self { 
            identifier:name,
            kind,
            value: value::Value { kind:value::ValueType::Null, value: String::from("Null") },
        }
    }
    //Function to set the value in a save way
    fn set_value(&mut self,value:value::Value)->bool{
        match value.kind{
            value::ValueType::Null=>{
                self.value=value;
                return true;
            }
            _=>{
                if self.value.kind==value::ValueType::Any||self.value.kind==value.kind{
                    self.value=value;
                    return true;
                }
                false
            }
        }
    }
    //Function to get the value
    fn get_value(&self)->value::Value{
        self.value.clone()
    }
}