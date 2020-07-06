fn main() {
    let input = String::from("LET foobar = 123");
    let mut lexer = lex::Lexer::new(&input);
    while lexer.peek() != '\0' {
        println!("{}", lexer.cur_char);
        lexer.next_char();
    }
}

pub mod lex;
pub mod parse;
pub mod emit;
