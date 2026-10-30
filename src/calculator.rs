use crate::{ value};
//Function to make addition
pub fn addition(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,true){
        return Err(false);
    }
    //Checking if any of this values is an String
    if matches!(val1.kind, value::ValueType::Text)
    || matches!(val2.kind, value::ValueType::Text){
        let value=value::Value::set_as_text(format!("{}{}",val1.value,val2.value));
        return Ok(value);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()+val2.value.parse::<f64>().unwrap();
    let value=value::Value::set_as_number(number.to_string());
    Ok(value)
}
//Function to make subtraction
pub fn subtraction(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()-val2.value.parse::<f64>().unwrap();
    let value=value::Value::set_as_number(number.to_string());
    Ok(value)
}
//Function to multiply
pub fn multiply(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    //Checking if any of the two values is a null or a boolean or a text
    if !checking_type_matematical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    //Action in case of numbers, because this two values have to be a Number
    let number: f64 = val1.value.parse::<f64>().unwrap()*val2.value.parse::<f64>().unwrap();
    let value=value::Value::set_as_number(number.to_string());
    Ok(value)
}
//Function to divide
pub fn divide(val1:&value::Value,val2:&value::Value,mode:&bool)->Result<value::Value,bool>{
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
    let value=value::Value::set_as_number(number.to_string());
    Ok(value)
}
//Function to make the module
pub fn module(val1:&value::Value,val2:&value::Value,mode:&bool)->Result<value::Value,bool>{
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
    let value=value::Value::set_as_number(number.to_string());
    Ok(value)
}
//Function to make the and
pub fn and(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    //Checking if there is only booleans
    if !checking_type_logical_operation(&val1.kind, &val2.kind, true){
        return Err(false);
    }
    //Checking if the two values are true
    if val1.is_a_true()&&val2.is_a_true(){
        return Ok(value::Value::set_as_true());
    }

    return Ok(value::Value::set_as_false());
}
//Function to make the or
pub fn or(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    //Checking if there is only booleans
    if !checking_type_logical_operation(&val1.kind, &val2.kind, true){
        return Err(false);
    }
    //Checking if the two values are true
    if val1.is_a_true()||val2.is_a_true(){
        return Ok(value::Value::set_as_true());
    }

    return Ok(value::Value::set_as_false());
}
//Function to make the not
pub fn not(val1:&value::Value)->Result<value::Value,bool>{
    if val1.kind!=value::ValueType::Boolean{
        return Err(false);
    }
    if val1.is_a_true(){
        return Ok(value::Value::set_as_false());
    }
    else{
        return Ok(value::Value::set_as_true());
    }
}
//Function to make equals
pub fn equals(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    if !checking_type_logical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    let result;
    if val1.is_a_number(){
        result=val1.value.parse::<f64>().unwrap()==val2.value.parse::<f64>().unwrap();
    }
    else{
        result=val1.value==val2.value;
    }
    match result{
        true=>return Ok(value::Value::set_as_true()),
        false=>return Ok(value::Value::set_as_false()),
    };
}
//Function to make greater
pub fn greater(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    if !checking_type_logical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    let result;
    if val1.is_a_number(){
        result=val1.value.parse::<f64>().unwrap()>val2.value.parse::<f64>().unwrap();
    }
    else{
        result=*val1.value>*val2.value;
    }
    match result{
        true=>return Ok(value::Value::set_as_true()),
        false=>return Ok(value::Value::set_as_false()),
    };
}
//Function to make lower
pub fn lower(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    if !checking_type_logical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    let result;
    if val1.is_a_number(){
        result=val1.value.parse::<f64>().unwrap()<val2.value.parse::<f64>().unwrap();
    }
    else{
        result=*val1.value<*val2.value;
    }
    match result{
        true=>return Ok(value::Value::set_as_true()),
        false=>return Ok(value::Value::set_as_false()),
    };
}
//Function to make greator or equal
pub fn greater_equal(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    if !checking_type_logical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    let result;
    if val1.is_a_number(){
        result=val1.value.parse::<f64>().unwrap()>=val2.value.parse::<f64>().unwrap();
    }
    else{
        result=*val1.value>=*val2.value;
    }
    match result{
        true=>return Ok(value::Value::set_as_true()),
        false=>return Ok(value::Value::set_as_false()),
    };
}
//Function to make lower or equal
pub fn lower_equal(val1:&value::Value,val2:&value::Value)->Result<value::Value,bool>{
    if !checking_type_logical_operation(&val1.kind,&val2.kind,false){
        return Err(false);
    }
    let result;
    if val1.is_a_number(){
        result=val1.value.parse::<f64>().unwrap()<=val2.value.parse::<f64>().unwrap();
    }
    else{
        result=*val1.value<=*val2.value;
    }
    match result{
        true=>return Ok(value::Value::set_as_true()),
        false=>return Ok(value::Value::set_as_false()),
    };
}
//Function to check if the type is correct for matematical operation
fn checking_type_matematical_operation(type1:&value::ValueType,type2:&value::ValueType,string_allowed:bool)->bool{
    !(matches!(*type1, value::ValueType::Null | value::ValueType::Boolean)
    || matches!(*type2, value::ValueType::Null | value::ValueType::Boolean)
    || (matches!(*type1, value::ValueType::Text)|| matches!(*type2, value::ValueType::Text)) && !string_allowed)
}
//Function to check if the type is correct for logical operation
fn checking_type_logical_operation(type1:&value::ValueType,type2:&value::ValueType,only_boolean:bool)->bool{
    match only_boolean{
        //Case for operator AND OR
        true=>{
            return matches!(*type1,value::ValueType::Boolean)&&matches!(*type2,value::ValueType::Boolean);
        }
        //Case for comparators, where they have to be the type of value
        false=>{
            return *type1 == *type2
                && matches!(
                    *type1,
                    value::ValueType::Number | value::ValueType::Text
                )
        }
    }
}
