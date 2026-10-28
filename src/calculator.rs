use crate::{ value};
//Function to make addition
pub fn addition(val1:value::Value,val2:value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean
    if matches!(val1.kind, value::ValueType::Null | value::ValueType::Boolean)
    || matches!(val2.kind, value::ValueType::Null | value::ValueType::Boolean){
        return Err(false);
    }
    //Checking if any of this values is an String
    if matches!(val1.kind, value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Text){
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
//Function to make subtraction
pub fn subtraction(val1:value::Value,val2:value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if matches!(val1.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text){
        return Err(false);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()-val2.value.parse::<f64>().unwrap();
    let value=value::Value{
        kind:value::ValueType::Number,
        value: number.to_string(),
    };
    Ok(value)
}
//Function to multiply
pub fn multiply(val1:value::Value,val2:value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if matches!(val1.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text){
        return Err(false);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()*val2.value.parse::<f64>().unwrap();
    let value=value::Value{
        kind:value::ValueType::Number,
        value: number.to_string(),
    };
    Ok(value)
}
//Function to divide
pub fn divide(val1:value::Value,val2:value::Value,mode:&bool)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if matches!(val1.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text){
        return Err(false);
    }
    //Checking if the value the second value is 0 and the save mode is on
    if val2.value.parse::<f64>().unwrap()==0.0&&*mode{
        return Err(true);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()/val2.value.parse::<f64>().unwrap();
    let value=value::Value{
        kind:value::ValueType::Number,
        value: number.to_string(),
    };
    Ok(value)
}
//Function to make the module
pub fn module(val1:value::Value,val2:value::Value,mode:&bool)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if matches!(val1.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Null | value::ValueType::Boolean | value::ValueType::Text){
        return Err(false);
    }
    //Checking if the value the second value is 0 and the save mode is on
    if val2.value.parse::<f64>().unwrap()==0.0&&*mode{
        return Err(true);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()%val2.value.parse::<f64>().unwrap();
    let value=value::Value{
        kind:value::ValueType::Number,
        value: number.to_string(),
    };
    Ok(value)
}