// emit.rs
// Emitter

pub trait Emitter {
    fn emit(&mut self, code: &str);
    fn emit_line(&mut self, code: &str);
    fn header_line(&mut self, code: &str);
    fn write_file(&self);
}

