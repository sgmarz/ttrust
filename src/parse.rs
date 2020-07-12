
use crate::lex::Lexer;
use crate::emit::Emitter;
use crate::token::{Token, TokenType};
use std::collections::BTreeSet;
pub struct Parser<'a, 'b> {
    lexer: &'a mut Lexer,
    emitter: &'b mut dyn Emitter,
    cur_token: Token,
    peek_token: Token,
    symbols: BTreeSet<String>,
    labels_declared: BTreeSet<String>,
    labels_gotoed: BTreeSet<String>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(lexer: &'a mut Lexer, emitter: &'b mut dyn Emitter) -> Self {
        let mut s = Self {
            lexer,
            emitter,
            cur_token: Token::default(),
            peek_token: Token::default(),
            symbols: BTreeSet::new(),
            labels_declared: BTreeSet::new(),
            labels_gotoed: BTreeSet::new(),
        };
        s.next_token();
        s.next_token();
        s
    }

    /// Check if the current token matches
    fn check_token(&self, kind: TokenType) -> bool {
        kind == self.cur_token.kind
    }
    /// Check the peek token
    // fn check_peek(&self, kind: TokenType) -> bool {
        // kind == self.peek_token.kind
    // }
    /// Advance to next token (or give an error)
    fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            abort!("Expected {:?}, got {:?}", kind, self.cur_token.kind);
        }
        self.next_token();
    }
    /// Get the next token from the lexer
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    // See if we are looking at the comparison operator
    fn is_comparison_operator(&self) -> bool {
        match self.cur_token.kind {
            TokenType::Gt | TokenType::GtEq | TokenType::Lt | TokenType::LtEq | TokenType::EqEq | TokenType::NotEq => true,
            _ => false
        }
    }


    // ////////////////////////
    // Lexer Interface
    // ////////////////////////

    /// The program itself token
    pub fn program(&mut self) {
        // println!("PROGRAM");
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void) {");
        // Skip preceding newlines
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
        // We have some "meat", see what it is
        while !self.check_token(TokenType::Eof) {
            self.statement();
        }

        self.emitter.emit_line("return 0;\n}");
        for label in self.labels_gotoed.iter() {
            if !self.labels_declared.contains(label) {
                abort!("Attempted to GOTO to undeclared label '{}'", label);
            }
        }
    }

    /// A particular statement in a program
    fn statement(&mut self) {
        match self.cur_token.kind {
            TokenType::Print => {
                // println!("STATEMENT-PRINT");
                self.next_token();
                if self.check_token(TokenType::String) {
                    let mut s = String::new();
                    s.push_str("printf(\"");
                    s.push_str(&self.cur_token.text);
                    s.push_str("\");");
                    self.emitter.emit_line(&s);
                    self.next_token();
                }
                else {
                    self.emitter.emit("printf(\"%.2f\", (float)(");
                    self.expression();
                    self.emitter.emit_line("));");
                }
            }
            TokenType::PrintLn => {
                // println!("STATEMENT-PRINT");
                self.next_token();
                if self.check_token(TokenType::String) {
                    let mut s = String::new();
                    s.push_str("printf(\"");
                    s.push_str(&self.cur_token.text);
                    s.push_str("\\n\");");
                    self.emitter.emit_line(&s);
                    self.next_token();
                }
                else {
                    self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                    self.expression();
                    self.emitter.emit_line("));");
                }
            }
            TokenType::If => {
                // println!("STATEMENT-IF");
                self.next_token();
                self.emitter.emit("if (");
                self.comparison();
                self.match_token(TokenType::Then);
                self.nl();
                self.emitter.emit_line(") {");
                while !self.check_token(TokenType::EndIf) {
                    self.statement();
                }
                self.match_token(TokenType::EndIf);
                self.emitter.emit_line("}");
            }
            TokenType::While => {
                // println!("STATEMENT-WHILE");
                self.next_token();
                self.emitter.emit("while (");
                self.comparison();
                self.match_token(TokenType::Repeat);
                self.nl();
                self.emitter.emit_line(") {");
                while !self.check_token(TokenType::EndWhile) {
                    self.statement();
                }  
                self.match_token(TokenType::EndWhile);
                self.emitter.emit_line("}");
            }
            TokenType::Label => {
                // println!("STATEMENT-LABEL");
                self.next_token();
                if self.labels_declared.contains(&self.cur_token.text) {
                    abort!("Label already declared '{}'", self.cur_token.text);
                }
                let mut s = String::from(&self.cur_token.text);
                s.push(':');
                self.emitter.emit_line(&s);
                self.match_token(TokenType::Ident);
            }
            TokenType::Goto => {
                // println!("STATEMENT-GOTO");
                self.next_token();
                self.labels_gotoed.insert(self.cur_token.text.clone());
                let mut s = String::from("goto ");
                s.push_str(&self.cur_token.text);
                s.push(';');
                self.emitter.emit_line(&s);
                self.match_token(TokenType::Ident);
            }
            TokenType::Let => {
                // println!("STATEMENT-LET");
                self.next_token();
                if !self.symbols.contains(&self.cur_token.text) {
                    self.symbols.insert(self.cur_token.text.clone());
                    let mut s = String::from("float ");
                    s.push_str(&self.cur_token.text);
                    s.push(';');
                    self.emitter.header_line(&s);
                }
                self.emitter.emit(&self.cur_token.text);
                self.emitter.emit(" = ");
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.expression();
                self.emitter.emit_line(";");
            }
            TokenType::Input => {
                // println!("STATEMENT-INPUT");
                self.next_token();
                if !self.symbols.contains(&self.cur_token.text) {
                    self.symbols.insert(self.cur_token.text.clone());
                    let mut s = String::from("float ");
                    s.push_str(&self.cur_token.text);
                    s.push(';');
                    self.emitter.header_line(&s);
                }
                self.emitter.emit("if (0 == scanf(\"%f\", &");
                self.emitter.emit(&self.cur_token.text);
                self.emitter.emit_line(")) {");
                self.emitter.emit(&self.cur_token.text);
                self.emitter.emit_line(" = 0;");
                self.emitter.emit_line("scanf(\"%*s\");");
                self.emitter.emit_line("}");
                self.match_token(TokenType::Ident);
            }
            _ => {
                abort!("Invalid statement ({:?})", self.cur_token.kind);
            }
        }
        self.nl();
    }

    /// A newline token in a statement
    fn nl(&mut self) {
        // println!("NEWLINE");
        self.match_token(TokenType::Newline);
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }
    /// An expression in a statement
    fn expression(&mut self) {
        // println!("EXPRESSION");
        self.term();

        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.term();
        }
    }

    /// A comparison operator
    fn comparison(&mut self) {
        // println!("COMPARISON");
        self.expression();

        if self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        }
        else {
            abort!("Expected comparison operator at {}.", self.cur_token.text);
        }
        while self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression();
        }
    }
    fn term(&mut self) {
        // println!("TERM");
        self.unary();

        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.unary();
        }
    }
    fn unary(&mut self) {
        // println!("UNARY");
        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        self.primary();
    }
    fn primary(&mut self) {
        // println!("PRIMARY ({})", self.cur_token.text);
        if self.check_token(TokenType::Number) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        else if self.check_token(TokenType::Ident) {
            if !self.symbols.contains(&self.cur_token.text) {
                abort!("Referencing variable before assignment '{}'", self.cur_token.text);
            }
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        else {
            abort!("Unexpected token at '{}'", self.cur_token.text);
        }
    }
}

