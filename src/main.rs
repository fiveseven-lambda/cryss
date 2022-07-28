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
        match parser::parse_sentence(&mut lexer) {
            Ok(Some((range, sentence))) => {
                let sentence: sentence::Sentence = sentence.into();
                println!("{}({0:?})\n{:#?}", range, sentence);
            }
            Ok(None) => {
                println!("end");
                break;
            }
            Err(error) => {
                error.eprint(lexer.log());
                return;
            }
        }
    }
}
