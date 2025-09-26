#[derive(Debug, PartialEq)]
enum TokenKind {
    Eof,
    Int(i64),
    LParen,
    Print,
    RParen,
    Str(String),
}

pub struct Token {
    kind: TokenKind,
    pos: usize,
}

pub struct Lexer<'a> {
    bytes: &'a [u8],
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            idx: 0,
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.bytes.get(self.idx).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.idx += 1;
        Some(c)
    }

    fn string(&mut self) -> String {
        let mut s = Vec::new();

        loop {
            let p = self.next();

            if p == Some(b'"') || p == None {
                break;
            };

            s.push(p.unwrap());
        }

        String::from_utf8(s).expect("String is not UTF-8")
    }

    fn number(&mut self) -> i64 {
        let start_pos = self.idx - 1;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.idx += 1;
            } else {
                break;
            }
        }

        println!("Possible number: {:?}", &self.bytes[start_pos..self.idx]);

        // zero-copy UTF-8 view
        let s = str::from_utf8(&self.bytes[start_pos..self.idx]).unwrap();
        i64::from_str_radix(s, 10).unwrap()
    }

    fn ignore(&mut self) {
        while let Some(c) = self.peek() {
            println!("Ignore fn: char {}", c as char);
            if c.is_ascii_whitespace() {
                self.idx += 1;
            } else {
                println!("Breaking, IDX at {}", self.idx);
                break;
            }
        }
    }

    fn lookup_identifier(&mut self) -> String {
        let start = self.idx - 1;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() {
                self.idx += 1;
            } else {
                break;
            }
        }
        String::from_utf8(self.bytes[start..self.idx].to_vec()).unwrap()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            self.ignore();
            let pos = self.idx;
            let peeked = self.next();
            if peeked.is_some() {
                println!("Peeked: {:?}", peeked.clone().unwrap() as char);
            };
            let token_kind = match peeked {
                None => TokenKind::Eof,
                Some(b'"') => TokenKind::Str(self.string()),
                Some(b'(') => TokenKind::LParen,
                Some(b')') => TokenKind::RParen,
                Some(c) if c.is_ascii_digit() => TokenKind::Int(self.number()),
                Some(c) if c.is_ascii_alphabetic() => match self.lookup_identifier().as_str() {
                    "print" => TokenKind::Print,
                    _ => panic!("Unknown identifier"),
                },
                u => panic!("Unknown token: {:?}", u),
            };
            println!("Token: {:?}", &token_kind);
            tokens.push(Token {
                kind: token_kind,
                pos: pos,
            });

            if tokens.last().unwrap().kind == TokenKind::Eof {
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
        let program = "         ";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_string() {
        let program = "\"hello\"";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens.first().unwrap().kind,
            TokenKind::Str(String::from("hello"))
        );
    }

    #[test]
    fn test_print_string() {
        let program = "print(\"hello\")";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Print);
        assert_eq!(tokens.get(1).unwrap().kind, TokenKind::LParen);
        assert_eq!(
            tokens.get(2).unwrap().kind,
            TokenKind::Str(String::from("hello"))
        );
        assert_eq!(tokens.get(3).unwrap().kind, TokenKind::RParen);
    }

    #[test]
    fn test_print_number() {
        let program = "print(1)";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Print);
        assert_eq!(tokens.get(1).unwrap().kind, TokenKind::LParen);
        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Int(1));
        assert_eq!(tokens.get(3).unwrap().kind, TokenKind::RParen);
    }

    #[test]
    fn test_string_and_ws() {
        let program = "      \"hello \" ";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens.first().unwrap().kind,
            TokenKind::Str(String::from("hello "))
        );
    }

    #[test]
    fn test_num() {
        let program = "12378";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Int(12378));
    }

    #[test]
    fn test_num_string_and_ws() {
        let program = " 12378 \"hello\" 3 \"world\"  ";
        let tokens = Lexer::new(&program).tokenize();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens.first().unwrap().kind, TokenKind::Int(12378));
        assert_eq!(
            tokens.get(1).unwrap().kind,
            TokenKind::Str(String::from("hello"))
        );
        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Int(3));
        assert_eq!(
            tokens.get(3).unwrap().kind,
            TokenKind::Str(String::from("world"))
        );
    }
}
