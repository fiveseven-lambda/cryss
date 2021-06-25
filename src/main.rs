//! CReate Your Sound from Scratch

mod compiler;
mod error;
mod lexer;
mod parser;
mod pos;
mod program;
mod sound;
mod syntax;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    let mut log = Vec::new();
    match parser::parse_expression(&mut lexer, &mut log) {
        Ok((result, end)) => {
            println!("{:#?}", result);
            println!("{:?}", end);
        }
        Err(err) => {
            err.print(&log);
        }
    }
}
