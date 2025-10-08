use std::{fs::File, io::Read};

use luffy::lexer::Lexer;
use luffy::parser::Parser;

fn main() {
    let mut s = String::new();
    let _ = File::open("examples/hello_world.lfy")
        .expect("File not found")
        .read_to_string(&mut s);
    let lex = Lexer::new(&s);
    println!("{:?}", &lex);
    let parser = Parser::new();
    println!("Parser => {:?}", parser);
}
