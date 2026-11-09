pub struct ParserError{
    pub kind: ErrorType,
    pub line: usize,
}
pub enum ErrorType{
    KeyWorldExpected,
    ValueExcepted,
    SemiColonExpected,
    OpenBracketExpected,
    CloseBrakectExpected,
    NumberExpected,
    OpenCurvyBrackedExpected,
    CloseCurvyBrackedExpected,
    ColonExpected,
    TypeExpected,
}