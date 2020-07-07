
use crate::lex::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut s = Self {
            lexer,
            cur_token: Token::sentinel(),
            peek_token: Token::sentinel(),
        };
        s.next_token();
        s.next_token();
        s
    }

    /// Check if the current token matches
    pub fn check_token(&self, kind: TokenType) -> bool {
        kind == self.cur_token.kind
    }
    /// Check the peek token
    pub fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.peek_token.kind
    }
    /// Advance to next token (or give an error)
    pub fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            panic!("Expected {:?}, got {:?}", kind, self.cur_token.kind);
        }
        self.next_token();
    }
    /// Get the next token from the lexer
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    /// The program itself token
    pub fn program(&mut self) {
        println!("PROGRAM");
        // Skip preceding newlines
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
        // We have some "meat", see what it is
        while !self.check_token(TokenType::Eof) {
            self.statement();
        }
    }

    /// A particular statement in a program
    pub fn statement(&mut self) {
        match self.cur_token.kind {
            TokenType::Print => {
                println!("STATEMENT-PRINT");
                self.next_token();
                if self.check_token(TokenType::String) {
                    self.next_token();
                }
                else {
                    self.expression();
                }
            }
            TokenType::If => {
                println!("STATEMENT-IF");
                self.next_token();
                self.comparison();
                self.match_token(TokenType::Then);
                self.nl();
                while !self.check_token(TokenType::EndIf) {
                    self.statement();
                }
                self.match_token(TokenType::EndIf);
            }
            TokenType::Let => {
                println!("STATEMENT-LET");
                self.next_token();
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.expression();
            }
            _ => {
                panic!("Invalid statement ({:?})", self.cur_token.kind);
            }
        }
        self.nl();
    }

    /// A newline token in a statement
    pub fn nl(&mut self) {
        println!("NEWLINE");
        self.match_token(TokenType::Newline);
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }
    /// An expression in a statement
    pub fn expression(&mut self) {
        println!("EXPRESSION");
        
    }
    pub fn comparison(&mut self) {
        println!("COMPARISON");
    }
}

