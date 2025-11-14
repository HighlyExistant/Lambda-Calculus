use crate::lexer::tokens::Token;

pub mod tokens;

pub trait Parse: Sized {
    fn parse(lexer: &mut Lexer) -> Option<Self>;
}

pub struct Lexer {
    buf: String,
    cursor: usize,
}

impl Lexer {
    pub fn new(buf: String) -> Self {
        Self { buf, cursor: 0usize }
    }
    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.parse_token() {
            tokens.push(token);
        }
        tokens
    }
    pub fn skip_whitespace(&mut self) {
        for c in self.buf.as_bytes()[self.cursor..self.buf.len()].iter().cloned() {
            if c != b' ' && c != b'\t' && c != b'\n' {
                break;
            }
            self.cursor += 1;
        }
    }
    pub fn is_next_punct(&self) -> bool {
        if self.cursor == self.buf.len() {
            return false;
        }
        let c = self.buf.as_bytes()[self.cursor];
        "~`!@#$%^&*-=+|;:',./\\?".contains(c as char)
    }
    pub fn is_next_literal(&self) -> bool {
        if self.cursor == self.buf.len() {
            return false;
        }
        let c = self.buf.as_bytes()[self.cursor];
        "1234567890\"\'".contains(c as char)
    }
    pub fn is_next_group(&self) -> bool {
        if self.cursor == self.buf.len() {
            return false;
        }
        let c = self.buf.as_bytes()[self.cursor];
        "({[<".contains(c as char)
    }
    pub fn is_next_ident(&self) -> bool {
        if self.cursor == self.buf.len() {
            return false;
        }
        let c = self.buf.as_bytes()[self.cursor];
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".contains(c as char)
    }
    pub fn get_char(&mut self) -> char {
        let c = self.buf.as_bytes()[self.cursor];
        c as char
    }
    pub fn next_char(&mut self) -> char {
        let c = self.buf.as_bytes()[self.cursor];
        self.cursor += 1;
        c as char
    }
    fn parse_token(&mut self) -> Option<Token> {
        Token::parse(self)
    }
}