//! CReate Your Sound from Scratch

// mod compiler;
// mod environment;
mod error;
mod lexer;
mod parser;
mod pos;
// mod program;
// mod sound;
mod syntax;
mod token;
// mod value;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    let mut log = Vec::new();

    loop {
        match parser::parse_expression(&mut lexer, &mut log) {
            Ok(result) => {
                println!("{:#?}", result);
            }
            Err(err) => {
                err.print(&log);
            }
        }
    }
}
