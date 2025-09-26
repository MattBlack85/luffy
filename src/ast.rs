#[derive(Debug)]
pub struct Program {
    parts: Vec<Stmt>,
}

#[derive(Debug)]
enum Stmt {
    Print(Expr),
}

#[derive(Debug)]
enum Expr {
    Str(String),
}
