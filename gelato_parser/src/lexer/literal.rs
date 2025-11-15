use std::fmt::Display;

use crate::lexer::{Lexer, Parse, span::Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitKind {
    Number,
    String,
    Character,
}
#[derive(Debug, Clone)]
pub struct Literal {
    pub span: Span,
    pub kind: LitKind,
    pub value: String
}
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.kind == other.kind
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.value).as_str())

    }
}
impl Parse for Literal {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let start = tokenizer.cursor;

        let mut literal = String::new();
        while tokenizer.is_next_literal() {
            literal.push(tokenizer.next_char());
        }
        let kind = match literal.chars().next().unwrap() {
            '\'' => LitKind::Character,
            '\"' => LitKind::String,
            _ => LitKind::Number
        };

        let end = tokenizer.cursor;
        Some(Literal {
            span: Span::new(start..end),
            kind,
            value: literal,
        })
    }
}