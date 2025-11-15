use std::fmt::Display;

use crate::lexer::{Lexer, Parse, span::Span};

#[derive(Debug, Clone)]
pub struct Ident {
    pub span: Span,
    pub ident: String,
}
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.ident).as_str())
    }
}
impl Parse for Ident {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let start = tokenizer.cursor;

        let mut ident = String::new();
        while tokenizer.is_next_ident() {
            ident.push(tokenizer.next_char());
        }

        let end = tokenizer.cursor;
        Some(Self { ident, span: Span::new(start..end) })
    }
}