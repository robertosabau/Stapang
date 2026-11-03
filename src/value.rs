#[derive(Clone,PartialEq,Debug)]
pub struct Value{
    pub kind: ValueType,
    pub value: String,
}
#[derive(PartialEq,Clone,Debug)]
pub enum ValueType{
    Text,
    Number,
    Boolean,
    Null,
    Any
}
impl Value{
    //Create a true value
    pub fn set_as_true()->Self{
        Self {
            kind: ValueType::Boolean,
            value: String::from("True")
        }
    }
    //Checking if it is a true
    pub fn is_a_true(&self)->bool{
        matches!(self.kind,ValueType::Boolean) && self.value == "True"
    }
    //Create a false value
    pub fn set_as_false()->Self{
        Self {
            kind: ValueType::Boolean,
            value: String::from("False")
        }
    }
    //Checking if it is a false
    pub fn is_a_false(&self)->bool{
        matches!(self.kind,ValueType::Boolean) && self.value == "False"
    }
    //Create a null value
    pub fn set_as_null()->Self{
        Self {
            kind: ValueType::Null,
            value: String::from("Null")
        }
    }
    //Create a number value
    pub fn set_as_number(value:String)->Self{
        Self {
            kind: ValueType::Number,
            value,
        }
    }
    //Checking if it is a number
    pub fn is_a_number(&self)->bool{
        matches!(self.kind,ValueType::Number)
    }
    //Create a text value
    pub fn set_as_text(value:String)->Self{
        Self {
            kind: ValueType::Text,
            value,
        }
    }
    //Checking if it is a text
    pub fn is_a_text(&self)->bool{
        matches!(self.kind,ValueType::Text)
    }
}