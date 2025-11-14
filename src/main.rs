use crate::{app::App, error::ASTError};

mod ast;
mod error;
mod app;
fn main() {
    let mut app = App::empty().unwrap();
    loop {
        let mut text = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut text) {
            continue;
        } else {
        }
        match app.input(&text) {
            Err(ASTError::EmptyTokenList) => {}
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
            _ => {}
        }
        while app.step().is_running() {
            println!("{}", app.statements());
            
        }
    }
}
