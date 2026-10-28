use crate::{ value};
//Function to make addition
pub fn addition(val1:value::Value,val2:value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean
    if (val1.kind==value::ValueType::Null||val2.kind==value::ValueType::Null)||(val1.kind==value::ValueType::Boolean||val2.kind==value::ValueType::Boolean){
        return Err(false);
    }
    //Checking if any of this values is an String
    if val1.kind==value::ValueType::Text||val2.kind==value::ValueType::Text{
        let value=value::Value{
            kind:value::ValueType::Text,
            value: format!("{}{}",val1.value,val2.value)
        };
        return Ok(value);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()+val2.value.parse::<f64>().unwrap();
    let value=value::Value{
        kind:value::ValueType::Number,
        value: number.to_string(),
    };
    Ok(value)
}