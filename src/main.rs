mod error;
mod lexer;
mod pos;
mod token;

fn main() {
    let mut lexer = lexer::Lexer::new(Box::new(std::io::BufReader::new(std::io::stdin())));
    loop {
        match lexer.next() {
            Ok(Some((range, token))) => {
                println!("{}({:?}) {:?}", range, range, token);
            }
            Ok(None) => break,
            Err(error) => {
                error.eprint(lexer.log());
                return;
            }
        }
    }
}
