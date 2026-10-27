pub struct EvaluatorError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    BooleanExpected,
    StackEmpty,
}