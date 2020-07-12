// emit.rs
// Emitter

use std::fs::File;
use std::io::prelude::*;

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Self {
        Self {
            full_path,
            header: String::new(),
            code: String::new(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }

    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.header.push('\n');
    }

    pub fn write_file(&self) {
        if let Ok(mut f) = File::create(&self.full_path) {
            // Use let _ to
            let _ = f.write_all(self.header.as_bytes());
            let _ = f.write_all(self.code.as_bytes());
            let _ = f.sync_all();
        }
        else {
            panic!("Could not open file for writing: '{}'", self.full_path);
        }
    }
}