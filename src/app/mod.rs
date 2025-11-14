use std::collections::LinkedList;

use gelato_parser::lexer::Lexer;

use crate::{app::state::{AppState, ControlFlow}, ast::Statements, error::ASTError};
mod state;

pub struct App {
    lexer: Lexer,
    pub state: Option<AppState>,
}

impl App {
    pub fn empty() -> Result<Self, ASTError> {
        let lexer = Lexer::new(String::new());
        let state = None;
        Ok(Self { lexer, state })
    }
    pub fn new(text: &str) -> Result<Self, ASTError> {
        let mut lexer = Lexer::new(text.to_string());
        let state = Some(AppState::new(&mut lexer)?);
        Ok(Self { lexer, state })
    }
    pub fn input(&mut self, text: &str) -> Result<(), ASTError> {
        self.lexer = Lexer::new(text.to_string());
        self.state = Some(AppState::new(&mut self.lexer)?);
        Ok(())
    }
    pub fn statements(&self) -> Statements {
        if let Some(state) = &self.state {
            state.statements.clone()
        } else {
            Statements { statements: LinkedList::new() }
        }
    }
    pub fn step(&mut self) -> ControlFlow {
        if let Some(state) = self.state.as_mut() {
            state.step()
        } else {
            ControlFlow::Finished
        }
    }
}