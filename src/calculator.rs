use crate::{ evaluator_error, value};
//Function to make addition
pub fn addition(val1:&value::Value,val2:&value::Value)->Result<value::Value,evaluator_error::ErrorType>{
    //Checking if any of the values are not valid
    match getting_values_aritmethic(val1, val2){
        Ok(pair) =>{
            match pair{
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_number(num1+num2))
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_text(format!("{}{}",text1,text2)))
                }
                _=> return Err(evaluator_error::ErrorType::InternarError),
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_number(num1-num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                _=> return Err(evaluator_error::ErrorType::InternarError),
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_number(num1*num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                _=> return Err(evaluator_error::ErrorType::InternarError),
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
                Pair::Numbers(num1, num2) =>{
                    if num2==0.0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_number(num1/num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                _=> return Err(evaluator_error::ErrorType::InternarError),
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
                Pair::Numbers(num1, num2) =>{
                    if num2==0.0{
                        return Err(evaluator_error::ErrorType::DivisionByZero);
                    }
                    return Ok(value::Value::set_as_number(num1%num2))
                }
                Pair::Texts(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType)
                }
                _=> return Err(evaluator_error::ErrorType::InternarError),
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1==num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1==text2));
                }
                Pair::Booleans(bool1, bool2) =>{
                    return Ok(value::Value::set_as_bool(bool1==bool2));
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1>num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1>text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1<num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1<text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1>=num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1>=text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
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
                Pair::Numbers(num1, num2) =>{
                    return Ok(value::Value::set_as_bool(num1<=num2));
                }
                Pair::Texts(text1, text2) =>{
                    return Ok(value::Value::set_as_bool(text1<=text2));
                }
                Pair::Booleans(_, _) =>{
                    return Err(evaluator_error::ErrorType::InvalidType);
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
        (value::ValueType::Number(num1),value::ValueType::Number(num2))=>{
            Ok(Pair::Numbers(*num1,*num2))
        }
        //Case all two are strings
        (value::ValueType::Text(text1),value::ValueType::Text(text2))=>{
            Ok(Pair::Texts(text1.clone(),text2.clone()))
        }
        //Case first one string
        (value::ValueType::Text(text),value::ValueType::Number(num))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        //Case second string
        (value::ValueType::Number(num),value::ValueType::Text(text))=>{
            Ok(Pair::Texts(text.clone(),String::from(num.to_string())))
        }
        _=>{
            Err(evaluator_error::ErrorType::InvalidType)
        }
    }
}
//Function to get the values if correct and an error in the other case for logical operation
fn getting_values_logical(val1:&value::Value,val2:&value::Value)->Result<Pair,evaluator_error::ErrorType>{
    match (&val1.kind,&val2.kind){
        //Case all two are numbers
        (value::ValueType::Number(num1),value::ValueType::Number(num2))=>{
            Ok(Pair::Numbers(*num1,*num2))
        }
        //Case all two are strings
        (value::ValueType::Text(text1),value::ValueType::Text(text2))=>{
            Ok(Pair::Texts(text1.clone(),text2.clone()))
        }
        //Case all two are booleans
        (value::ValueType::Boolean(bool1),value::ValueType::Boolean(bool2))=>{
            Ok(Pair::Booleans(*bool1,*bool2))
        }
        _=>{
            Err(evaluator_error::ErrorType::InvalidType)
        }
    }
}
//Enum for pairs
enum Pair {
    Numbers(f64, f64),
    Texts(String, String),
    Booleans(bool,bool)
}