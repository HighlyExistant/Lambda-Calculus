use std::{fmt::{Display, Write}, io::{Cursor, Read}};

use crate::lexer::{Lexer, Parse, delimiter::Delimiter, ident::Ident, literal::Literal, punct::Punct, span::Span};

#[derive(Debug, Clone)]
pub enum Token {
    Literal(Literal),
    Ident(Ident),
    Punct(Punct),
    Group(Delimiter),
}
// #if = (\b.\t.\f.b t f)
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Group(group) => {
                f.write_str(format!("{}", group).as_str())
            }
            Token::Ident(ident) => {
                f.write_str(format!("{}", ident).as_str())
            }
            Token::Literal(literal) => {
                f.write_str(format!("{}", literal).as_str())
            }
            Token::Punct(punct) => {
                f.write_str(format!("{}", punct).as_str())
            }
        }
    }
}

impl Token {
    pub fn span(&self) -> &Span {
        match self {
            Token::Group(g) => &g.span,
            Token::Ident(ident) => &ident.span,
            Token::Literal(literal) => &literal.span,
            Token::Punct(punct) => &punct.span,
        }
    }
    pub fn is_literal(&self) -> bool {
        if let Self::Literal(_) = self {
            return true;
        }
        false
    }
    pub fn is_ident(&self) -> bool {
        if let Self::Ident(_) = self {
            return true;
        }
        false
    }
    pub fn is_group(&self) -> bool {
        if let Self::Group(_) = self {
            return true;
        }
        false
    }
    pub fn is_punct(&self) -> bool {
        if let Self::Punct(_) = self {
            return true;
        }
        false
    }
    pub fn is_punct_subset(&self, c: &str) -> bool {
        if let Self::Punct(p) = self {
            return p.punct == c;
        }
        false
    }
    pub fn get_literal(&self) -> Option<Literal> {
        if let Self::Literal(lit) = self {
            return Some(lit.clone());
        }
        None
    }
    pub fn get_group(&self) -> Option<Delimiter> {
        if let Self::Group(del) = self {
            return Some(del.clone());
        }
        None
    }
    pub fn get_ident(&self) -> Option<Ident> {
        if let Self::Ident(ident) = self {
            return Some(ident.clone());
        }
        None
    }
    pub fn get_punct(&self) -> Option<Punct> {
        if let Self::Punct(punct) = self {
            return Some(punct.clone());
        }
        None
    }
}

impl Parse for Token {
    fn parse(tokenizer: &mut Lexer) -> Option<Token> {
        tokenizer.skip_whitespace();
        if tokenizer.is_next_punct() {
            return Some(Self::Punct(Punct::parse(tokenizer)?));
        }
        if tokenizer.is_next_ident() {
            return Some(Self::Ident(Ident::parse(tokenizer)?));
        }
        if tokenizer.is_next_literal() {
            return Some(Self::Literal(Literal::parse(tokenizer)?));
        }
        if tokenizer.is_next_group() {
            return Some(Self::Group(Delimiter::parse(tokenizer)?));
        }
        None
    }
}
#[derive(Debug, Clone)]
pub struct Tokens {
    pub tokens: Vec<Token>,
    pub next: usize,

}
impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.tokens.get(self.next)?;
        self.next += 1;
        Some(next.clone())
    }
}
impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = &self.tokens[self.next..self.tokens.len()];
        // let mut prev_span = self.next..self.tokens.len();
        // let mut current_span = self.next..self.tokens.len();
        for i in slice {
            // current_span = i.span().range.clone();
            // for _ in 0..(current_span.start-prev_span.end) {
            //     f.write_char(' ')?;
            //     println!("prev: {prev_span:?} curr: {current_span:?}");
            // }
            f.write_str(format!("{}", i).as_str())?;
            // prev_span = current_span;
        }
        Ok(())
    }
}