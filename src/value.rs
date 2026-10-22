pub struct Value{
    pub kind: ValueType,
    pub value: String,
}
#[derive(PartialEq)]
pub enum ValueType{
    Text,
    Number,
    Boolean,
    Null,
    Any
}