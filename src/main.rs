use gelato_parser::lexer::Lexer;

use crate::{app::App, ast::{FromTokens, Statement, Statements}};

mod ast;
mod error;
mod app;
fn main() {
    let str = "(\\b.\\t.\\f.b t f)(\\x.\\y.y) a b";
    let mut app = App::new(str).unwrap();
    println!("\n\n\n");
    println!("{}", app.statements);
    while !app.step() {
        println!("{}", app.statements);
    }
}
