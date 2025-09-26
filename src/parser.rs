use crate::lexer::{Lexer, Token};

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(prog: &str) -> Self {
        Self {
            tokens: Lexer::new(prog).tokenize(),
        }
    }
}
