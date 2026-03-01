use crate::app::App;

mod ast;
mod error;
mod app;
fn main() {
    let mut app = App::new();
    app.run();
}