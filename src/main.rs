use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use crate::emit::Emitter;

#[macro_export]
macro_rules! abort
{
	() => ({
           print!("Compiler error\n");
           std::process::exit(-1)
		   });
	($fmt:expr) => ({
            print!(concat!("Compiler error: ", concat!($fmt, "\n")));
            std::process::exit(-1)
			});
	($fmt:expr, $($args:tt)+) => ({
            print!(concat!("Compiler error: ", concat!($fmt, "\n")), $($args)+);
            std::process::exit(-1)
			});
}
fn main() {
    let a: Vec<_> = args().collect();
    if a.len() < 2 {
        abort!("Not enough arguments provided.");
    }
    // println!("Teeny tiny written in Rust.");
    let path = a[1].clone();
    let mut f = File::open(path).expect("Unable to open file.");
    let mut input = String::new();
    let read_result = f.read_to_string(&mut input);
    if read_result.is_err() {
        abort!("Unable to read input file.");
    }
    // println!("Read {} bytes.", read_result.ok().unwrap());
    let mut lexer = lex::Lexer::new(&input);
    let mut emitter = cemitter::CEmitter::new(String::from("out.c"));
    let mut parser = parse::Parser::new(&mut lexer, &mut emitter);
    parser.program();
    emitter.write_file();
}

pub mod lex;
pub mod parse;
pub mod emit;
pub mod token;
pub mod cemitter;