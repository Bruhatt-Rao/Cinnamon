use std::env;
use std::fs;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = lexer::parse_config(&args);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let tokens = lexer::tokenize(contents);
}