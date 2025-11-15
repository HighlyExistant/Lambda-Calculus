use crate::{app::{App, calculator::Calculator}, ast::remove_multiple_whitespace, error::ASTError};

mod ast;
mod error;
mod app;
fn main() {
    
    let mut app = App::new();
    app.run();
}