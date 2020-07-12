// cemitter.rs
// The C emitter
// Stephen Marz

use crate::emit::Emitter;
use std::fs::File;
use std::io::prelude::*;

pub struct CEmitter {
    full_path: String,
    header: String,
    code: String,
}

impl CEmitter {
    pub fn new(full_path: String) -> Self {
        Self {
            full_path,
            header: String::new(),
            code: String::new(),
        }
    }
}
impl Emitter for CEmitter {
    fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }

    fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.header.push('\n');
    }

    fn write_file(&self) {
        if let Ok(mut f) = File::create(&self.full_path) {
            // Use let _ to avoid checking the result.
            let _ = f.write_all(self.header.as_bytes());
            let _ = f.write_all(self.code.as_bytes());
            let _ = f.sync_all();
        }
        else {
            panic!("Could not open file for writing: '{}'", self.full_path);
        }
    }
}