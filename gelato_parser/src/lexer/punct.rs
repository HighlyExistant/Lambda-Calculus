use std::fmt::Display;

use crate::lexer::{Lexer, Parse, span::Span};


#[derive(Debug, Clone)]
pub struct Punct {
    pub span: Span,
    pub punct: String,
}
impl PartialEq for Punct {
    fn eq(&self, other: &Self) -> bool {
        self.punct == other.punct
    }
}
impl Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.punct).as_str())
    }
}
impl Punct {
    pub fn is_punct(&self, str: &str) -> bool {
        self.punct == str
    }
}
impl Parse for Punct {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let start = tokenizer.cursor;

        let mut punct = String::new();
        if tokenizer.is_next_punct() {
            punct.push(tokenizer.next_char());
        }

        let end = tokenizer.cursor;
        Some(Self { punct, span: Span::new(start..end) })
    }
}