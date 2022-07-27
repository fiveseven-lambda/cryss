mod error;
mod expr;
mod lexer;
mod parser;
mod pos;
mod sentence;
mod token;
mod types;

fn main() {
    let mut lexer = lexer::Lexer::new(Box::new(std::io::BufReader::new(std::io::stdin())), true);
    loop {
        match parser::parse_expr(&mut lexer) {
            Ok(Some((range, expr))) => {
                let expr: expr::Expr = expr.into();
                println!("{}({0:?}) {:?}", range, expr);
            }
            Ok(None) => break,
            Err(error) => {
                error.eprint(lexer.log());
                return;
            }
        }
    }
}
