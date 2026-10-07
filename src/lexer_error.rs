pub struct LexerError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    UnclosedString,
    InavalidCaracter,
    InvalidIndentifier
}