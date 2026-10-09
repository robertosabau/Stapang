pub struct Value{
    kind: ValueType,
    value: String,
}

pub enum ValueType{
    Text,
    Number,
    Boolean,
    Null
}