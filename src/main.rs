fn main() {
    let input = String::from("IF+-123 foo*THEN/");
    let mut lexer = lex::Lexer::new(&input);
    let mut token = lexer.get_token();
    while token.kind != token::TokenType::Eof {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }
}

pub mod lex;
pub mod parse;
pub mod emit;
pub mod token;
