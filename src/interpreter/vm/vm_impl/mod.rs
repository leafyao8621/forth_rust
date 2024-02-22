use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use super::VM;
use super::VMData;
pub mod parser;
pub mod instruction;

impl VMData {
    pub fn new() -> VMData {
        VMData { main_stack: Vec::new(), control_stack: Vec::new() }
    }
}

impl VM {
    pub fn new() -> VM {
        VM { code: Vec::new(), data: VMData::new() }
    }
    pub fn load(&mut self, file_name: &str) {
        let path = Path::new(file_name);
        let display = path.display();
        let mut fin = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(fin) => fin,
        };
        let mut src = Vec::new();
        match fin.read_to_end(&mut src) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(fin) => fin,
        };
        let mut parser = parser::Parser::new(src);
        parser.parse(self);
    }
    pub fn log(&self) {
        for i in &self.code {
            i.log();
        }
    }
    pub fn execute(&mut self) {
        self.code[0].execute(&mut self.data);
    }
}
