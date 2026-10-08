pub struct ParserError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    UnclosedString,
    InvalidCharacter,
    InvalidIndentifier
}