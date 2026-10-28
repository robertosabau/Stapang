use crate::{ value};
//Function to make addition
pub fn addition(val1:value::Value,val2:value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,true){
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
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
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
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
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
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
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
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
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
//Function to check if the type is correct for matematical operation
fn checking_type_matematical_operation(type1:&value::ValueType,type2:&value::ValueType,string_allowed:bool)->bool{
    matches!(*type1, value::ValueType::Null | value::ValueType::Boolean)
    || matches!(*type2, value::ValueType::Null | value::ValueType::Boolean)
    || (matches!(*type1, value::ValueType::Text)|| matches!(*type2, value::ValueType::Text)) && !string_allowed
}