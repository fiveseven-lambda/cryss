//! program.rs を実行する環境

use crate::program;

struct Environment {}

impl Environment {
    fn new() -> Environment {
        Environment {}
    }
    fn run(&mut self, statement: program::Statement) {}
}
