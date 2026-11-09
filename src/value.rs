#[derive(Clone,PartialEq,Debug)]
pub struct Value{
    pub kind: ValueType,
}
#[derive(PartialEq,Clone,Debug)]
pub enum ValueType{
    Text(String),
    Number(f64),
    Boolean(bool),
    Null,
}
impl Value{
    //Create a true value
    pub fn set_as_true()->Self{
        Self {
            kind: ValueType::Boolean(true),
        }
    }
    //Checking if it is a true
    pub fn is_a_true(&self)->bool{
        matches!(self.kind,ValueType::Boolean(true))
    }
    //Create a false value
    pub fn set_as_false()->Self{
        Self {
            kind: ValueType::Boolean(false),
        }
    }
    //Checking if it is a false
    pub fn is_a_false(&self)->bool{
        matches!(self.kind,ValueType::Boolean(false))
    }
    //Create a null value
    pub fn set_as_null()->Self{
        Self {
            kind: ValueType::Null
        }
    }
    //Create a boolean
    pub fn set_as_bool(value:bool)->Self{
        Self { kind: ValueType::Boolean(value) }
    }
    //Checking if it is a null
    pub fn is_a_null(&self)->bool{
        matches!(self.kind,ValueType::Null)
    }
    //Create a number value
    pub fn set_as_number(value:f64)->Self{
        Self {
            kind: ValueType::Number(value)
        }
    }
    //Create a text value
    pub fn set_as_text(value:String)->Self{
        Self {
            kind: ValueType::Text(value)
        }
    }
}