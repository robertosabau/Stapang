use crate::{value};
pub struct Array{
    identifier: String,
    kind: value::ValueType,
    values: Vec<value::Value>,
    positions:usize,
}