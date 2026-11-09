#[derive(Clone,PartialEq,Debug)]
pub struct Value{
    pub kind: ValueType,
}
#[derive(PartialEq,Clone,Debug)]
pub enum ValueType{
    Text(String),
    Index(usize),
    Int(i64),
    Float(f64),
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
    //Create a index value
    pub fn set_as_index(value:usize)->Self{
        Self {
            kind: ValueType::Index(value)
        }
    }
    //Create a int value
    pub fn set_as_int(value:i64)->Self{
        Self {
            kind: ValueType::Int(value)
        }
    }
    //Create a float value
    pub fn set_as_float(value:f64)->Self{
        Self {
            kind: ValueType::Float(value)
        }
    }
    //Create a text value
    pub fn set_as_text(value:String)->Self{
        Self {
            kind: ValueType::Text(value)
        }
    }
}