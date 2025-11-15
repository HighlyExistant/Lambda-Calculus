use std::collections::LinkedList;

use gelato_parser::lexer::Lexer;

use crate::{app::state::{CalculatorState, ControlFlow}, ast::Statements, error::ASTError};
pub struct Calculator {
    lexer: Lexer,
    pub state: Option<CalculatorState>,
}

impl Calculator {
    pub fn empty() -> Self {
        let lexer = Lexer::new(String::new());
        let state = None;
        Self { lexer, state }
    }
    pub fn new(text: &str) -> Result<Self, ASTError> {
        let mut lexer = Lexer::new(text.to_string());
        let state = Some(CalculatorState::new(&mut lexer)?);
        Ok(Self { lexer, state })
    }
    pub fn input(&mut self, text: &str) -> Result<(), ASTError> {
        self.lexer = Lexer::new(text.to_string());
        self.state = Some(CalculatorState::new(&mut self.lexer)?);
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