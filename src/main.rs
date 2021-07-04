//! CReate Your Sound from Scratch

mod compiler;
mod environment;
mod error;
mod lexer;
mod parser;
mod pos;
mod program;
mod sound;
mod syntax;
mod token;
mod types;
mod value;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    let mut log = Vec::new();

    let mut environment = environment::Environment::new();

    loop {
        match parser::parse_statement(&mut lexer, &mut log) {
            Ok(Some(statement)) => match environment.run(statement) {
                Ok(()) => {}
                Err(err) => break err.print(&log),
            },
            Ok(None) => break,
            Err(err) => break err.print(&log),
        }
    }
}
