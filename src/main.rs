//! CReate Your Sound from Scratch

mod compiler;
mod environment;
mod error;
mod function;
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
    let matches = clap::App::new("cryss")
        .arg(clap::Arg::with_name("input"))
        .get_matches();

    let mut lexer = match matches.value_of("input") {
        Some(filename) => lexer::Lexer::new(
            Box::new(std::io::BufReader::new(
                std::fs::File::open(filename).expect("cannot open the input file"),
            )),
            false,
        ),
        None => lexer::Lexer::new(Box::new(std::io::BufReader::new(std::io::stdin())), true),
    };
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
