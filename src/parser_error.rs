pub struct ParserError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    KeyWorldExpected,
    UnclosedString,
    InvalidCharacter,
    InvalidIndentifier
}