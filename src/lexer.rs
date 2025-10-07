use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Char,
    Str { terminated: bool },
    Int,
    Float,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Ident,
    Literal {
        kind: LiteralKind,
        suffix_start: u32,
    },
    Eof,

    // Operators
    Plus,
    Minus,
    Star,
    Eq,
    Lt,
    Gt,
    And,
    Or,
    OpenParen,
    CloseParen,
    Comma,
    Dot,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Ws,
    Unknown,
    Semi,
}

const EOF_CHAR: char = '\0';

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    len_remaining: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.is_eof() {
            self.bump();
        }
    }

    fn eat_ident(&mut self) {
        self.eat_while(|c| c.is_alphabetic() || c == '_');
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn number(&mut self) -> () {
        self.eat_decimal_digits();
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek() {
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn is_str_terminated(&mut self) -> bool {
        while let Some(c) = self.bump() {
            match c {
                '"' => return true,
                _ => (),
            }
        }

        false
    }

    fn eat_string(&mut self) -> TokenKind {
        let terminated = self.is_str_terminated();
        let suffix_start = self.pos_within_token();
        if terminated {
            self.eat_ident();
        }
        let kind = LiteralKind::Str { terminated };
        TokenKind::Literal { kind, suffix_start }
    }

    fn advance_token(&mut self) -> Token {
        let Some(c) = self.bump() else {
            return Token::new(TokenKind::Eof, 0);
        };

        let token_kind = match c {
            c if c.is_ascii_whitespace() => {
                self.eat_while(|c| c.is_ascii_whitespace());
                TokenKind::Ws
            }
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '=' => TokenKind::Eq,
            '&' => TokenKind::And,
            '|' => TokenKind::Or,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            ';' => TokenKind::Semi,
            '0'..'9' => {
                self.number();
                let suffix_start = self.pos_within_token();
                TokenKind::Literal {
                    kind: LiteralKind::Int,
                    suffix_start: suffix_start,
                }
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                self.eat_ident();
                TokenKind::Ident
            }
            '"' => self.eat_string(),
            _ => TokenKind::Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.advance_token();
            tokens.push(token);

            if token.kind == TokenKind::Eof {
                break;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ws() {
        let program = "   ";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_numbers() {
        let program = "12345";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens.first().unwrap().kind,
            TokenKind::Literal {
                kind: LiteralKind::Int,
                suffix_start: 5,
            }
        );
    }

    #[test]
    fn test_ident() {
        let program = "hello world";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 4);
        let first = tokens.first().unwrap();
        let second = tokens.get(2).unwrap();
        assert_eq!(first.kind, TokenKind::Ident);
        assert_eq!(first.len, 5);
        assert_eq!(second.kind, TokenKind::Ident);
        assert_eq!(second.len, 5);

        let program = "hello hello_world";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 4);
        let first = tokens.first().unwrap();
        let second = tokens.get(2).unwrap();
        assert_eq!(first.kind, TokenKind::Ident);
        assert_eq!(first.len, 5);
        assert_eq!(second.kind, TokenKind::Ident);
        assert_eq!(second.len, 11);

        let program = "hello _world";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 4);
        let first = tokens.first().unwrap();
        let second = tokens.get(2).unwrap();
        assert_eq!(first.kind, TokenKind::Ident);
        assert_eq!(first.len, 5);
        assert_eq!(second.kind, TokenKind::Ident);
        assert_eq!(second.len, 6);
    }

    #[test]
    fn test_program() {
        let program = "func main() { jet_pistol(\"one piece\"); }";
        let tokens = Lexer::new(&program).tokenize();
        assert_eq!(tokens.len(), 16);
    }
}
