use crate::{ value};
pub struct Array{
    identifier: String,
    kind: value::ValueType,
    values: Vec<value::Value>,
    positions:usize,
}
impl Array {
    //Constructor of a Array
    pub fn new(name:String,kind:value::ValueType,position:usize)->Self{
        Self { 
            identifier:name,
            kind,
            values:vec![value::Value {
                kind: value::ValueType::Null,
                value: String::from("Null"),
            };
            position],
            positions:position,
        }
    }//Function to set the value in a save way
    pub fn set_value(&mut self,value:value::Value,position:usize)->bool{
        match value.kind{
            value::ValueType::Null=>{
                self.values[position]=value;
                return true;
            }
            _=>{
                if self.kind==value::ValueType::Any||self.kind==value.kind{
                    self.values[position]=value;
                    return true;
                }
                false
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