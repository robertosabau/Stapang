#[derive(Clone)]
pub struct Value{
    pub kind: ValueType,
    pub value: String,
}
#[derive(PartialEq,Clone)]
pub enum ValueType{
    Text,
    Number,
    Boolean,
    Null,
    Any
}