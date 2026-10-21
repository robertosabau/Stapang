pub struct Value{
    pub kind: ValueType,
    pub value: String,
}

pub enum ValueType{
    Text,
    Number,
    Boolean,
    Null,
    Any
}