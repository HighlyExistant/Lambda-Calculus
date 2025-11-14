use std::collections::LinkedList;

use gelato_parser::lexer::Lexer;

use crate::{ast::{Abstraction, FromTokens, Group, Statement, Statements}, error::ASTError};

pub struct App {
    lexer: Lexer,
    pub statements: Statements,
}

impl App {
    pub fn new(text: &str) -> Result<Self, ASTError> {
        let mut lexer = Lexer::new(text.to_string());
        let tokens = lexer.parse();
        let mut tokens_iter = tokens.iter().cloned();
        let statements = Statements::from_tokens(&mut tokens_iter)?;
        Ok(Self { lexer, statements })
    }
    /// replaces all variables in the abstraction that aren't renamed
    /// in sub abstraction.
    pub fn replace_names(statements: &mut Statements, from: &str, to: &Statement) {
        for statement in statements.statements.iter_mut() {
            match statement {
                Statement::Abstraction(abstraction) => {
                    if abstraction.variable.name.ident == from { // If the variable has been renamed then ignore
                        continue;
                    } else { // rename all variables in the sub abstraction
                        Self::replace_names(&mut abstraction.next, from, to); 
                    }
                }
                Statement::Group(group) => {
                    Self::replace_names(&mut group.statements, from, to); 
                }
                Statement::Term(term) => {
                    if to.is_abstraction() {
                        let mut statements = LinkedList::new();
                        statements.push_back(to.clone());
                        *statement = Statement::Group(Group { statements: Statements { statements } });
                        return;
                    }
                    if term.name.ident == from {
                        *statement = to.clone();
                    }
                }
            }
        }
    }
    pub fn push_statements(statements: &mut Statements, rhs: &Statements) {
        for statement in rhs.statements.iter().rev().cloned() {
            statements.statements.push_front(statement);
        }
    }
    fn apply_abstraction(parent_group: &mut Group, abstraction: &mut Abstraction, input: Option<Statement>) -> bool {
        if let Some(input) = input {
            // If it has input, apply the abstraction to replace the names
            // then push the statements of the abstraction onto the group
            Self::replace_names(&mut abstraction.next, &abstraction.variable.name.ident, &input);
            // Check if its being curried
            if abstraction.next.statements.front().unwrap().is_abstraction() {
                parent_group.statements.statements.push_front(Statement::Group(Group { statements: abstraction.next.clone() }));
            } else {
                Self::push_statements(&mut parent_group.statements, &abstraction.next);
            }
            return false;
        } else {
            // If it does not have input, it is done running
            parent_group.statements.statements.push_front(Statement::Abstraction(abstraction.clone()));
            return true;
        };
    }
    /// Assumes it is running within a group
    /// Returns true when it is done running
    fn application(parent_statements: &mut Statements, group: &mut Group) -> bool {
        let input = parent_statements.statements.pop_front();
        // Pop 1
        let front = group.statements.statements.pop_front().expect("A group cannot be empty");
        let ret = match front {
            Statement::Abstraction(mut abstraction) => {
                Self::apply_abstraction(group, &mut abstraction, input)
            }
            Statement::Group(mut g) => {
                // If it has another group inside, execute internally then remove parenthesis
                // let internal_input = group.statements.statements.pop_front();
                // println!("input {internal_input:?}");
                Self::application(&mut group.statements, &mut g);
                false
            }
            Statement::Term(t) => {
                // If it has a single term inside, you are done
                group.statements.statements.push_front(Statement::Term(t));
                if let Some(input) = input {
                    group.statements.statements.push_front(input);
                }
                true
            }
        };
        Self::push_statements(parent_statements, &group.statements);
        ret
    }
    fn abstraction_step(abstraction: &mut Abstraction) -> bool {
        if let Some(next) = abstraction.next_curried_mut() {
            let pop = next.next.statements.pop_front().unwrap();
            if let Statement::Group(mut group) = pop.clone() {
                Self::application(&mut next.next, &mut group);
                return false;
            }
            next.next.statements.push_front(pop);
            return Self::abstraction_step(next);
        }
        true
    }
    
    pub fn step(&mut self) -> bool {
        let statement = if let Some(s) = self.statements.statements.pop_front() {
            s
        } else {
            return true;
        };
        match statement {
            Statement::Abstraction(mut abstraction) => {
                // push onto stack
                let ret = Self::abstraction_step(&mut abstraction);
                self.statements.statements.push_front(Statement::Abstraction(abstraction));
                return ret;
            }
            Statement::Term(_) => {
                self.statements.statements.push_front(statement);
                return true;
            }
            Statement::Group(mut group) => {
                Self::application(&mut self.statements, &mut group);
                return false;
            }
        }
    }
}