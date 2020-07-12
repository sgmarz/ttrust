// emit.rs
// Emitter

use crate::token::TokenType;

pub trait Emitter {
    fn begin(&mut self);
    fn end(&mut self);

    fn emit_op(&mut self, op: TokenType);
    fn emit_val(&mut self, val: &str);

    fn emit_print(&mut self, val: &str);
    fn emit_input(&mut self, val: &str);

    fn write_file(&self);
}

