use crate::value;
#[derive(PartialEq,Clone,Debug)]
pub enum ValueSource{
    Literal(value::Value),
    Identifier(Identifier)
}
#[derive(PartialEq,Clone,Debug)]
pub enum Identifier{
    Variable(String),
    Array{
        name:String,
        position:Box<ValueSource>,
    },
}