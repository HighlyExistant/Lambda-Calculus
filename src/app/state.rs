use std::collections::LinkedList;

use gelato_parser::lexer::Lexer;

use crate::{ast::{Abstraction, FromTokens, Group, Statement, Statements, Term}, error::ASTError};

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlFlow {
    Finished,
    Running,
}
impl ControlFlow {
    pub fn is_running(self) -> bool {
        self == Self::Running
    }
}

pub struct AppState {
    pub statements: Statements,
}

impl AppState {
    pub fn new(lexer: &mut Lexer) -> Result<Self, ASTError> {
        let tokens = lexer.parse();
        let mut tokens_iter = tokens.iter().cloned();
        let statements = Statements::from_tokens(&mut tokens_iter)?;
        Ok(Self { statements })
    }
    /// replaces all variables in the abstraction that aren't renamed
    /// in sub abstraction.
    pub fn replace_names(statements: &mut Statements, from: &Term, to: &Statement) {
        // Iterate over all the statements to change any terms inside of them
        // that match with 'from', with 'to'
        for statement in statements.statements.iter_mut() {
            match statement {
                // If the variable is within another abstraction then we need to check for
                // 1. If the variable has been renamed in another abstraction, then ignore
                // 2. If the variable has not been renamed, replace all variables that match 
                Statement::Abstraction(abstraction) => {
                    if &abstraction.variable == from { // If the variable has been renamed then ignore
                        continue;
                    } else { // rename all variables in the sub abstraction
                        Self::replace_names(&mut abstraction.next, from, to); 
                    }
                }
                Statement::Group(group) => {
                    Self::replace_names(&mut group.statements, from, to); 
                }
                Statement::Term(term) => {
                    // If the variable matches with a term, and to is an abstraction
                    // we can replace it in a group.
                    if to.is_abstraction() {
                        let mut statements = LinkedList::new();
                        statements.push_back(to.clone());
                        *statement = Statement::Group(Group { statements: Statements { statements } });
                    } else if term == from {
                        *statement = to.clone();
                    }
                }
            }
        }
    }
    /// Pushes a list of statements rhs, onto the front of the list of statements.
    /// This preserves the ordering of the elements on rhs.
    pub fn push_statements_front(statements: &mut Statements, rhs: &Statements) {
        // Due to the stack like behaviour of linked lists, we reverse the iterator
        // to preserve the element order.
        for statement in rhs.statements.iter().rev().cloned() {
            statements.statements.push_front(statement);
        }
    }
    fn apply_abstraction(parent_group: &mut Group, abstraction: &mut Abstraction, input: Option<Statement>) -> ControlFlow {
        if let Some(input) = input {
            // If it has input, apply the abstraction to replace the names
            // then push the statements of the abstraction onto the group
            Self::replace_names(&mut abstraction.next, &abstraction.variable, &input);
            // Check if its being curried
            if abstraction.next.statements.front().unwrap().is_abstraction() {
                parent_group.statements.statements.push_front(Statement::Group(Group { statements: abstraction.next.clone() }));
            } else {
                Self::push_statements_front(&mut parent_group.statements, &abstraction.next);
            }
            return ControlFlow::Running;
        } else {
            // If it does not have input, it is done running
            parent_group.statements.statements.push_front(Statement::Abstraction(abstraction.clone()));
            return ControlFlow::Finished;
        };
    }
    /// Assumes it is running within a group. This will apply a lambda abstraction
    /// that is within a group.
    fn application(parent_statements: &mut Statements, group: &mut Group) -> ControlFlow {
        let input = parent_statements.statements.pop_front();
        let front = group.statements.statements.pop_front().expect("A group cannot be empty");
        let ret = match front {
            Statement::Abstraction(mut abstraction) => {
                Self::apply_abstraction(group, &mut abstraction, input)
            }
            Statement::Group(mut g) => {
                // If it has another group inside, execute internally
                Self::application(&mut group.statements, &mut g);
                ControlFlow::Running
            }
            Statement::Term(t) => {
                // If it has a single term inside, you are done
                group.statements.statements.push_front(Statement::Term(t));
                if let Some(input) = input {
                    group.statements.statements.push_front(input);
                }
                ControlFlow::Finished
            }
        };
        // Remove parenthesis by pushing the statements of group onto its parent.
        // This effectively marks the end of the application of the lambda abstract.
        Self::push_statements_front(parent_statements, &group.statements);
        ret
    }
    // In the case of nested lambda abstracts that can still be executed.
    fn currying_step(abstraction: &mut Abstraction) -> ControlFlow {
        if let Some(next) = abstraction.next_curried_mut() {
            let pop = next.next.statements.pop_front().unwrap();
            if let Statement::Group(mut group) = pop.clone() {
                Self::application(&mut next.next, &mut group);
                return ControlFlow::Running;
            }
            next.next.statements.push_front(pop);
            return Self::currying_step(next);
        }
        ControlFlow::Finished
    }
    
    /// Processes a single step of the program.
    pub fn step(&mut self) -> ControlFlow {
        let statement = if let Some(s) = self.statements.statements.pop_front() {
            s
        } else { // If the program contains nothing, then it is finished running
            return ControlFlow::Finished;
        };
        match statement {
            Statement::Abstraction(mut abstraction) => {
                let ret = Self::currying_step(&mut abstraction);
                self.statements.statements.push_front(Statement::Abstraction(abstraction));
                return ret;
            }
            Statement::Term(_) => {
                self.statements.statements.push_front(statement);
                return ControlFlow::Finished;
            }
            Statement::Group(mut group) => {
                Self::application(&mut self.statements, &mut group);
                return ControlFlow::Running;
            }
        }
    }
}