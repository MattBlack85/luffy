//use crate::ast::*;
use crate::lexer::Token;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Token,
}
type PResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}
