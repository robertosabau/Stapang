pub struct EvaluatorError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    BooleanExpected,
    StackEmpty,
    VariableNotDefined{name:String},
    ArrayNotDefined{name:String},
    InternarError,
    OutOfBound,
    InvalidType,
    DivisionByZero,
    ReadError,
    NumericConversionError,
    InvalidValue,
    VariableNotInicializated{name:String},
    ArrayNotInicializated{name:String,position:usize},
}