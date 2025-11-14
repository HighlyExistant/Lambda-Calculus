use std::io::{Cursor, Read};

use crate::lexer::{Lexer, Parse};

#[derive(Debug, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Character(char),
}
impl Parse for Literal {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let mut literal = String::new();
        while tokenizer.is_next_literal() {
            literal.push(tokenizer.next_char());
        }
        if literal.chars().next().unwrap() == '\"' {
            return Some(Literal::String(literal));
        }
        Some(Literal::Number(literal))
    }
}
#[derive(Debug, Clone)]
pub struct Punct {
    pub punct: String,
}
impl Punct {
    pub fn is_punct(&self, str: &str) -> bool {
        self.punct == str
    }
}
impl Parse for Punct {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let mut punct = String::new();
        if tokenizer.is_next_punct() {
            punct.push(tokenizer.next_char());
        }
        Some(Self { punct })
    }
}
#[derive(Debug, Clone)]
pub struct Delimiter {
    pub open: char,
    pub close: char,
    pub tokens: Vec<Token>,
}
impl Parse for Delimiter {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let open = tokenizer.next_char();
        let mut del = Delimiter {
            open,
            close: '\0',
            tokens: vec![]
        };
        while let Some(token) = tokenizer.parse_token() {
            let get = tokenizer.get_char();
            del.tokens.push(token);
            match open {
                '(' => if get == ')' { 
                    del.close = get;
                    break; 
                }
                '{' => if get == '}' { 
                    del.close = get;
                    break; 
                }
                '<' => if get == '>' { 
                    del.close = get;
                    break; 
                }
                '[' => if get == ']' { 
                    del.close = get;
                    break; 
                }
                _ => {}
            }
        }
        tokenizer.cursor += 1;
        Some(del)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub ident: String,
}
impl Parse for Ident {
    fn parse(tokenizer: &mut Lexer) -> Option<Self> {
        let mut ident = String::new();
        while tokenizer.is_next_ident() {
            ident.push(tokenizer.next_char());
        }
        Some(Self { ident })
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Literal(Literal),
    Ident(Ident),
    Punct(Punct),
    Group(Delimiter),
}

impl Token {
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