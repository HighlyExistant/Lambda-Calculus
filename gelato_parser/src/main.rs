use crate::lexer::Lexer;

mod lexer;
fn main() {
    let str = String::from("(v x)(x<0>.0)");
    let mut tokenizer = Lexer::new(str);
    println!("{:#?}", tokenizer.parse());
    println!("{:#?}", tokenizer.parse());
}
