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
    let arg_vec: Vec<_> = args().collect();
    if arg_vec.len() < 2 {
        abort!("Not enough arguments provided.");
    }
    // println!("Teeny tiny written in Rust.");
    let mut in_path = String::new();
    let mut out_path = String::from("out.c");
    let mut i = 1;
    while i < arg_vec.len() {
        let a = &arg_vec[i];
        if a.get(0..1) == Some("-") {
            match a.get(..).unwrap() {
                "-o" => {
                    i += 1;
                    if i >= arg_vec.len() {
                        abort!("No output file specified to -o switch.");
                    }
                    out_path = arg_vec[i].clone();    
                }
                _ => abort!("Unknown switch {}", a),
            }
        }
        else {
            if in_path.len() != 0 {
                abort!("Input file name specified more than once.");
            }
            in_path = a.clone();
        }
        i += 1;
    }
    if in_path.len() < 1 {
        abort!("No filename specified.");
    }
    let mut f = File::open(&in_path).expect("Unable to open file.");
    let mut input = String::new();
    let read_result = f.read_to_string(&mut input);
    if read_result.is_err() {
        abort!("Unable to read input file.");
    }
    // println!("Read {} bytes.", read_result.ok().unwrap());
    let mut lexer = lex::Lexer::new(&input);
    let mut emitter = cemitter::CEmitter::new(out_path);
    let mut parser = parse::Parser::new(&mut lexer, &mut emitter);
    parser.program();
    emitter.write_file();
}

pub mod lex;
pub mod parse;
pub mod emit;
pub mod token;
pub mod cemitter;