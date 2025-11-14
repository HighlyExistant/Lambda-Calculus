use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ASTError {
    #[error("Syntax error: {0}")]
    Syntax(String),
    // Used to mark the end of reading statements
    #[error("Finished Reading")]
    EmptyTokenList,
}