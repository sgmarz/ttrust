
use crate::token::{Token, TokenType};
pub struct Lexer {
    source: String,
    pub cur_char: char,
    cur_pos: i32,
}

impl Lexer {
    /// Create a new lexer to analyze the given input
    pub fn new(input: &String) -> Self {
        let mut s = Self {
            source: input.clone() + "\n",
            cur_char: ' ' as char,
            cur_pos: -1,
        };
        s.next_char();
        s
    }

    /// Skip to the next character
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        let p = self.cur_pos as usize;
        if p >= self.source.len() {
            self.cur_char = '\0';
        }
        else {
            self.cur_char = self.source.as_bytes()[p] as char;
        }
    }

    /// Return the next character without consuming it.
    pub fn peek(&self) -> char {
        let p = self.cur_pos as usize + 1;
        if p >= self.source.len() {
            '\0'
        }
        else {
            self.source.as_bytes()[p] as char
        }
    }

    /// Get the type of token we're looking at
    pub fn get_token(&mut self) -> Token {
        // Build a string that will be the token itself
        let mut s = String::new();
        s.push(self.cur_char);
        
        // Determine the token type
        let token_type = 
            match self.cur_char {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Asterisk,
                '/' => TokenType::Slash,
                '\n' => TokenType::Newline,
                '\0' => TokenType::Eof,
                _ => TokenType::Unknown
            };
        self.next_char();
        Token::new(&s, token_type)
    }
}