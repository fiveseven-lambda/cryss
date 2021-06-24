//! CReate Your Sound from Scratch

mod error;
mod lexer;
mod pos;
mod program;
mod sound;
mod syntax;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(std::io::BufReader::new(std::io::stdin()), true);
    let mut log = Vec::new();
    loop {
        match lexer.next(&mut log) {
            Ok(Some((pos, token))) => {
                println!("{:?} ({})", token, pos);
            }
            Ok(None) => break,
            Err(err) => {
                err.print(&log);
                break;
            }
        }
    }
}
