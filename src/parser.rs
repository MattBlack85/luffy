//use crate::ast::*;
use crate::lexer::{Token, TokenKind};

#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}
type PResult<T> = Result<T, ParseError>;

#[derive(Clone, Copy, Debug)]
pub struct SpanToken {
    kind: TokenKind,
    span: Span,
}

pub struct Parser<'s> {
    src: &'s str,
    pub toks: Vec<SpanToken>, // includes Ws; parser will skip them
    i: usize,
}

impl<'s> Parser<'s> {
    pub fn new(src: &'s str, tokens: Vec<Token>) -> Self {
        // turn len-only tokens into spanned tokens (absolute byte offsets)
        let mut tkns: Vec<SpanToken> = Vec::with_capacity(tokens.len());
        let mut off: usize = 0;

        for t in tokens {
            let len = t.len as usize;
            tkns.push(SpanToken {
                kind: t.kind,
                span: Span {
                    start: off,
                    end: off + len,
                },
            });
            off += len;
        }
        Self {
            src,
            toks: tkns,
            i: 0,
        }
    }
}

impl<'s> std::fmt::Debug for Parser<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in &self.toks {
            match t.kind {
                TokenKind::Eof | TokenKind::Ws => continue,
                _ => {
                    let Span { start, end } = t.span;
                    write!(f, "Kind: {:?} => {}\n", t.kind, &self.src[start..end])?
                }
            }
        }
        Ok(())
    }
}
