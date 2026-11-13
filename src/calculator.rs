use crate::{ evaluator_error, value};
//Function to make addition
pub fn addition(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_float(num1+num2))
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_text(format!("{}{}",text1,text2)))
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_uint(num1+num2));
                }
                Pair::Integers(num1,num2) =>{
                    return Ok(value::Value::set_as_int(num1+num2));
                }
            }
        }
        Err(error) => return Err(error),
    }
}
//Function to make subtraction 
pub fn subtraction(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_float(num1-num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_uint(num1-num2));
                }
                Pair::Integers(num1,num2) =>{
                    return Ok(value::Value::set_as_int(num1-num2));
                }
            }
        }
        Err(error) => return Err(error),
    }
}
//Function to make multiplication 
pub fn multiply(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_float(num1*num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_uint(num1*num2));
                }
                Pair::Integers(num1,num2) =>{
                    return Ok(value::Value::set_as_int(num1*num2));
                }
            }
        }
        Err(error) => return Err(error),
    }
}
//Function to divide
pub fn divide(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    if num2==0.0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_float(num1/num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                Pair::Index(num1, num2) => {
                    if num2==0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_uint(num1/num2));
                }
                Pair::Integers(num1,num2) =>{
                    if num2==0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_int(num1/num2));
                }
            }
        }
        Err(error) => return Err(error),
    }
}
//Function to make the module
pub fn module(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    if num2==0.0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_float(num1%num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InternarError);
                }
                Pair::Index(num1, num2) => {
                    if num2==0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_uint(num1/num2));
                }
                Pair::Integers(num1,num2) =>{
                    if num2==0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_int(num1/num2));
                }
            }
        }
        Err(error) => return Err(error),
    }
}
//Function to make the and
pub fn and(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
                match pair{
                    Pair::Booleans(bool1, bool2) =>{
                    return Ok(value::Value::set_as_bool(bool1&&bool2));
                }
                _=>{ 
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                }
            }
        Err(error) =>return Err(error),
    }
}
//Function to make the or
pub fn or(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
                match pair{
                    Pair::Booleans(bool1, bool2) =>{
                    return Ok(value::Value::set_as_bool(bool1||bool2));
                }
                _=>{ 
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                }
            }
        Err(error) =>return Err(error),
    }
}
//Function to make the not
pub fn not(val1:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match val1.kind{
        value::ValueType::Boolean(val) => return Ok(value::Value::set_as_bool(val)),
        _=> return Err(evaluator_error::ErrorType::InvalidType)
    }
}
//Function to make the equals
pub fn equals(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1==num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1==text2));
                }
                Pair::Booleans(bool1, bool2) =>{
                    return Ok(value::Value::set_as_bool(bool1==bool2));
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1==num2));
                }
                Pair::Integers(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1==num2));
                }
            }
    }
        Err(error) => return Err(error),
    }
}
//Function to make greater
pub fn greater(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1>num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1>text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1>num2));
                }
                Pair::Integers(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1>num2));
                }
            }
    }
        Err(error) => return Err(error),
    }
}
//Function to make less
pub fn less(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1<num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1<text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1<num2));
                }
                Pair::Integers(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1<num2));
                }
            }
    }
        Err(error) => return Err(error),
    }
}
//Function to make greater
pub fn greater_equal(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1>=num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1>=text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1>=num2));
                }
                Pair::Integers(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1>=num2));
                }
            }
    }
        Err(error) => return Err(error),
    }
}
//Function to make less
pub fn less_equal(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    match getting_values_logical(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Floats(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1<=num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1<=text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
                }
                Pair::Index(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1<=num2));
                }
                Pair::Integers(num1, num2) => {
                    return Ok(value::Value::set_as_bool(num1<=num2));
                }
            }
    }
        Err(error) => return Err(error),
    }
}
//Function to get the values if correct and an error in the other case for aritmetic calculs
fn getting_values_aritmethic(val1:&value::Value,val2:&value::Value)->Result<Pair,evaluator_error::ErrorType>{
    match (&val1.kind,&val2.kind){
        //Case all two are numbers
        (value::ValueType::Float(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1,*num2))
        }
        //Case all two are strings
        (value::ValueType::Text(text1),value::ValueType::Text(text2))=>{
            Ok(Pair::Texts(text1.clone(),text2.clone()))
        }
        //Cases one is a text an the other a float
        (value::ValueType::Text(text),value::ValueType::Float(num))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        (value::ValueType::Float(num),value::ValueType::Text(text))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        //Cases one is a text an the other a int
        (value::ValueType::Text(text),value::ValueType::Int(num))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        (value::ValueType::Int(num),value::ValueType::Text(text))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        //Cases one is a text an the other a index
        (value::ValueType::Text(text),value::ValueType::UInt(num))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        (value::ValueType::UInt(num),value::ValueType::Text(text))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        //Cases one is a index and the other a int
        (value::ValueType::Int(num1),value::ValueType::UInt(num2))=>{
            Ok(Pair::Integers(*num1,i64::try_from(*num2).unwrap()))
        }
        (value::ValueType::UInt(num1),value::ValueType::Int(num2))=>{
            Ok(Pair::Integers(i64::try_from(*num1).unwrap(),*num2))
        }
        //Cases one is a index and the other a float
        (value::ValueType::Float(num1),value::ValueType::UInt(num2))=>{
            Ok(Pair::Floats(*num1,*num2 as f64))
        }
        (value::ValueType::UInt(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1 as f64,*num2))
        }
        //Cases one is a int and the other a float
        (value::ValueType::Float(num1),value::ValueType::Int(num2))=>{
            Ok(Pair::Floats(*num1,*num2 as f64))
        }
        (value::ValueType::Int(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1 as f64,*num2))
        }
        _=>{
            Err(evaluator_error::ErrorType::InvalidType)
        }
    }
}
//Function to get the values if correct and an error in the other case for logical operation
fn getting_values_logical(val1:&value::Value,val2:&value::Value)->Result<Pair,evaluator_error::ErrorType>{
    match (&val1.kind,&val2.kind){
        //Case all two are floats
        (value::ValueType::Float(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1,*num2))
        }
        //Case all two are strings
        (value::ValueType::Text(text1),value::ValueType::Text(text2))=>{
            Ok(Pair::Texts(text1.clone(),text2.clone()))
        }
        //Case all two are booleans
        (value::ValueType::Boolean(bool1),value::ValueType::Boolean(bool2))=>{
            Ok(Pair::Booleans(*bool1,*bool2))
        }
        //Case all two are index
        (value::ValueType::UInt(num1),value::ValueType::UInt(num2))=>{
            Ok(Pair::Index(*num1,*num2))
        }
        //Case all two are integers
        (value::ValueType::Int(num1),value::ValueType::Int(num2))=>{
            Ok(Pair::Integers(*num1,*num2))
        }
        //Cases one is a index and the other a int
        (value::ValueType::Int(num1),value::ValueType::UInt(num2))=>{
            Ok(Pair::Integers(*num1,i64::try_from(*num2).unwrap()))
        }
        (value::ValueType::UInt(num1),value::ValueType::Int(num2))=>{
            Ok(Pair::Integers(i64::try_from(*num1).unwrap(),*num2))
        }
        //Cases one is a index and the other a float
        (value::ValueType::Float(num1),value::ValueType::UInt(num2))=>{
            Ok(Pair::Floats(*num1,*num2 as f64))
        }
        (value::ValueType::UInt(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1 as f64,*num2))
        }
        //Cases one is a int and the other a float
        (value::ValueType::Float(num1),value::ValueType::Int(num2))=>{
            Ok(Pair::Floats(*num1,*num2 as f64))
        }
        (value::ValueType::Int(num1),value::ValueType::Float(num2))=>{
            Ok(Pair::Floats(*num1 as f64,*num2))
        }
        _=>{
            Err(evaluator_error::ErrorType::InvalidType)
        }
    }
}
//Enum for pairs
enum Pair {
    Index(usize,usize),
    Integers(i64,i64),
    Floats(f64, f64),
    Texts(String, String),
    Booleans(bool,bool)
}