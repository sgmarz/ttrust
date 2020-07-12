
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
        // Skip whitespace
        self.skip_whitespace();
        // Skip comments
        self.skip_comment();
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
                '=' => if self.peek() == '=' {
                           self.next_char();
                           s.push(self.cur_char);
                           TokenType::EqEq
                       }
                       else {
                           TokenType::Eq
                       },
                '>' =>  if self.peek() == '=' {
                            self.next_char();
                            s.push(self.cur_char);
                            TokenType::GtEq
                        }
                        else {
                            TokenType::Gt
                        },
                '<' =>  if self.peek() == '=' {
                            self.next_char();
                            s.push(self.cur_char);
                            TokenType::LtEq
                        }
                        else {
                            TokenType::Lt
                        },
                '!' =>  if self.peek() == '=' {
                            self.next_char();
                            s.push(self.cur_char);
                            TokenType::NotEq
                        }
                        else {
                            abort!("Expected !=, got !{}", self.peek());
                        },
                '"' => {
                    self.next_char();
                    let start_pos = self.cur_pos;
                    while self.cur_char != '"' {
                        match self.cur_char {
                            '\r' | '\n' | '\t' | '\\' | '%' => {
                                abort!("Illegal character in string.");
                            }
                            _ => {
                                self.next_char();
                            }
                        }
                    }
                    let tok_text = self.source.get(start_pos as usize..self.cur_pos as usize).unwrap();
                    s = String::from(tok_text);
                    TokenType::String
                },
                '0'..='9' => {
                    let start_pos = self.cur_pos;
                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                    if self.peek() == '.' {
                        self.next_char();
                        if !self.peek().is_ascii_digit() {
                            abort!("Illegal character in number.");
                        }
                        while self.peek().is_ascii_digit() {
                            self.next_char();
                        }
                    }
                    let tok_text = self.source.get(start_pos as usize..(self.cur_pos as usize + 1)).unwrap();
                    s = String::from(tok_text);
                    TokenType::Number
                }
                'a'..='z' | 'A'..='Z' => {
                    let start_pos = self.cur_pos;
                    while self.peek().is_ascii_alphanumeric() {
                        self.next_char();
                    }
                    let tok_text = self.source.get(start_pos as usize..(self.cur_pos as usize + 1)).unwrap();
                    let keyword = Token::check_if_keyword(tok_text);
                    if keyword == TokenType::Unknown {
                        // identifier
                        s = String::from(tok_text);
                        TokenType::Ident
                    }
                    else {
                        s = String::from(tok_text);
                        keyword
                    }
                }
                '\n' => TokenType::Newline,
                '\0' => TokenType::Eof,
                _ => abort!("Unknown token: '{}'", self.cur_char),
            };
        self.next_char();
        Token::new(&s, token_type)
    }

    /// Skip whitespace
    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }
}