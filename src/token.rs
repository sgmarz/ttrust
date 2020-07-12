#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum TokenType {
    Unknown = -2,
    Eof = -1,
    Newline = 0,
    Number,
    Ident,
    String,
    // Keywords
    Label = 101,
    Goto,
    Print,
    Input,
    Let,
    If,
    Then,
    EndIf,
    While,
    Repeat,
    EndWhile,
    // Operators
    Eq = 201,
    Plus,
    Minus,
    Asterisk,
    Slash,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

/// Rust will not compare token types by default
/// Here we convert into the integer equivalent first
/// and then see if those integers are equal.
impl PartialEq for TokenType {
    fn eq(&self, rhs: &TokenType) -> bool {
        let l = *self as i32;
        let r = *rhs as i32;
        l == r
    }
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    /// Create a new token with a given text and type. We can't
    /// use the keyword type, so we use kind here.
    pub fn new(token_text: &String, kind: TokenType) -> Self {
        Self {
            text: token_text.clone(),
            kind
        }
    }

    /// Return the token type of a given string keyword
    pub fn check_if_keyword(token_text: &str) -> TokenType {
        match token_text {
            "LABEL" => TokenType::Label,
            "GOTO" => TokenType::Goto,
            "PRINT" => TokenType::Print,
            "INPUT" => TokenType::Input,
            "LET" => TokenType::Let,
            "IF" => TokenType::If,
            "THEN" => TokenType::Then,
            "ENDIF" => TokenType::EndIf,
            "WHILE" => TokenType::While,
            "REPEAT" => TokenType::Repeat,
            "ENDWHILE" => TokenType::EndWhile,
            _ => TokenType::Unknown,
        }
    }
}

/// When we create a new Token, we'd like to have a sentinel,
/// uninitialized value. However, Rust wants something, so 
/// it has the Default trait.
impl Default for Token {
    fn default() -> Self {
        Self {
            text: String::new(),
            kind: TokenType::Unknown,
        }
    }
}
