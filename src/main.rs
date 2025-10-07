use std::{fs::File, io::Read};

use luffy::lexer::Lexer;
use luffy::parser::Parser;

fn main() {
    let mut s = String::new();
    let _ = File::open("examples/hello_world.luffy")
        .expect("File not found")
        .read_to_string(&mut s);
    let tokens = Lexer::new(&s).tokenize();
    println!("Lexed: {:?}", &tokens);
    let parser = Parser::new(&s, tokens);
    println!("Parser => {:?}", parser);
}
