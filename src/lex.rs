pub struct Lexer {
    source: String,
    pub cur_char: char,
    cur_pos: i32,
}

impl Lexer {
    pub fn new(input: &String) -> Self {
        let mut s = Self {
            source: input.clone() + "\n",
            cur_char: ' ' as char,
            cur_pos: -1,
        };
        s.next_char();
        s
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        let p = self.cur_pos as usize;
        if p > self.source.len() {
            self.cur_char = '\0';
        }
        else {
            self.cur_char = self.source.as_bytes()[p] as char;
        }
    }

    pub fn peek(&self) -> char {
        let p = self.cur_pos as usize + 1;
        if p >= self.source.len() {
            '\0'
        }
        else {
            self.source.as_bytes()[p] as char
        }
    }

}