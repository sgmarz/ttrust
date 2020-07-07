use std::io::prelude::*;
use std::fs::File;
use std::env::args;

fn main() {
    let a: Vec<_> = args().collect();
    if a.len() < 2 {
        panic!("Not enough arguments provided.");
    }
    println!("Teeny tiny written in Rust.");
    let path = a[1].clone();
    let mut f = File::open(path).expect("Unable to open file.");
    let mut input = String::new();
    let read_result = f.read_to_string(&mut input);
    if read_result.is_err() {
        panic!("Unable to read input file.");
    }
    println!("Read {} bytes.", read_result.ok().unwrap());
    let mut lexer = lex::Lexer::new(&input);
    let mut parser = parse::Parser::new(&mut lexer);
    parser.program();
    println!("Parsing complete.");

}

pub mod lex;
pub mod parse;
pub mod emit;
pub mod token;
