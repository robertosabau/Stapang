use crate::value;
pub struct Variable{
    pub identifier: String,
    pub kind: value::ValueType,
    pub value: value::Value,
}
impl Variable{
    fn set_value(mut self,value:value::Value)->bool{
        match self.kind{
            value::ValueType::Any=>{
                self.value=value;
                return true;
            }
            _=>{
                if self.value.kind==value::ValueType::Null{
                    self.value=value;
                    return true;
                }
                false
            }
        }
    }
}