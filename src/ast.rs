use std::{collections::LinkedList, fmt::Display};

use gelato_parser::lexer::{ident::Ident, punct::Punct, span::Span, tokens::{Token, Tokens}};

use crate::error::ASTError;

pub trait FromTokens: Sized {
    fn from_tokens(tokens: &mut Tokens) -> Result<Self, ASTError>;
}

/// λvariable.next
#[derive(Debug, Clone)]
pub struct Abstraction {
    pub variable: Term,
    pub next: Statements,
}
impl Abstraction {
    pub fn next_curried(&self) -> Option<&Abstraction> {
        if let Statement::Abstraction(abs) = self.next.statements.front().unwrap() {
            Some(abs)
        } else {
            None
        }
    }
    pub fn next_curried_mut(&mut self) -> Option<&mut Abstraction> {
        if let Statement::Abstraction(abs) = self.next.statements.front_mut().unwrap() {
            Some(abs)
        } else {
            None
        }
    }
}
impl Display for Abstraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("λ{}.{}", self.variable, self.next))
    }
}
/// term
#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub name: Ident,
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.name.ident))
    }
}
#[derive(Debug, Clone)]
pub struct Group {
    pub statements: Statements,
}
impl Group {
    pub fn only_contains_terms(&self) -> bool {
        for statement in self.statements.statements.iter() {
            if statement.is_abstraction() {
                return false;
            }
        }
        true
    }
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({})", self.statements))
    }
}
/// S ::= λ<term>.<optional-statement> | 
/// <term> = string | 
/// (S ...) 
#[derive(Debug, Clone)]
pub enum Statement {
    Abstraction(Abstraction),
    Group(Group),
    Term(Term)
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Abstraction(abstraction) => {
                abstraction.fmt(f)
            }
            Statement::Group(group) => {
                group.fmt(f)
            }
            Statement::Term(term) => {
                term.fmt(f)
            }
        }
    }
}
impl Statement {
    pub fn is_abstraction(&self) -> bool {
        if let Self::Abstraction(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_group(&self) -> bool {
        if let Self::Group(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_term(&self) -> bool {
        if let Self::Term(_) = self {
            true
        } else {
            false
        }
    }
}
impl FromTokens for Statement {
    fn from_tokens(tokens: &mut Tokens) -> Result<Self, ASTError> {
        match tokens.next().ok_or(ASTError::EmptyTokenList)? {
            Token::Punct(punct) => { // Creating an Abstraction
                if punct.punct.as_str() == "\\" {
                    let variable = tokens
                        .next().ok_or(ASTError::Syntax("Expected Variable".to_string()))?
                        .get_ident().ok_or(ASTError::Syntax("Expected Identifier".to_string()))?;
                    if !tokens
                        .next().ok_or(ASTError::Syntax("Expected Punt".to_string()))?
                        .get_punct().ok_or(ASTError::Syntax("Expected Punct".to_string()))?.is_punct(".") {
                        // check if syntax is ok
                        return Err(ASTError::Syntax("Failed to locate '.'".to_string()));
                    }
                    let next = Statements::from_tokens(tokens)?;
                    if next.statements.is_empty() {
                        return Err(ASTError::Syntax("There needs to be statements after an abstraction".to_string()));
                    }
                    return Ok(Statement::Abstraction(Abstraction { 
                        variable: Term { name: variable }, 
                        next,
                    }));
                } else { // No punct other than '\'
                    println!("FAILED");
                    return Err(ASTError::Syntax("Failed to find lambda".to_string()));
                }
            }
            Token::Ident(ident) => {
                return Ok(Self::Term(Term { name: ident }));
            }
            Token::Group(mut group) => {
                // let mut tokens = group.tokens.iter().cloned();
                return Ok(Self::Group(Group { statements: Statements::from_tokens(&mut group.tokens)? }));
            }
            _ => return Err(ASTError::Syntax("Invalid Syntax".to_string())),
        }
    }
}
pub fn remove_multiple_whitespace(str: String) -> String {
    let mut ret = String::new();
    let mut whitespace_prev = false;
    for c in str.chars() {
        if c.is_whitespace() {
            if !whitespace_prev {
                ret.push(c);
            }
            whitespace_prev = true;
        } else {
            ret.push(c);
            whitespace_prev = false;
        }
    }
    ret
}
#[derive(Debug, Clone)]
pub struct Statements {
    pub statements: LinkedList<Statement>,
}
impl Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        
        for statement in self.statements.iter() {
            str.push_str(format!("{}", statement).as_str());
            str.push(' ');
            // statement.fmt(f)?;
        }
        let _ = str.pop();
        str = remove_multiple_whitespace(str);
        f.write_str(format!("{}", str).as_str())?;
        Ok(())
    }
}
impl FromTokens for Statements {
    fn from_tokens(tokens: &mut Tokens) -> Result<Self, ASTError> {
        let mut statements = LinkedList::new();
        loop {
            match Statement::from_tokens(tokens) {
                Ok(statement) => {
                    statements.push_back(statement);
                }
                Err(ASTError::EmptyTokenList) => {
                    return Ok(Self { statements });
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}