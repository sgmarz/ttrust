fn main() {
    let input = String::from("+- */");
    let mut lexer = lex::Lexer::new(&input);
    let mut token = lexer.get_token();
    while token.kind != token::TokenType::Eof {
        println!("{}", token.kind as i32);
        token = lexer.get_token();
    }
}

pub mod lex;
pub mod parse;
pub mod emit;
pub mod token;
