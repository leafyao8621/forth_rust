use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use self::parser::Parser;

use super::VMCode;
use super::VM;
use super::VMData;
pub mod parser;
pub mod instruction;

impl VMData {
    pub fn new() -> VMData {
        VMData {
            ip: 0,
            compile: false,
            main_stack: Vec::new(),
            control_stack: Vec::new()
        }
    }
}

impl VMCode {
    pub fn new() -> VMCode {
        VMCode { compiled: Vec::new(), interpreted: Vec::new() }
    }
}

impl VM {
    pub fn new() -> VM {
        VM { parser: Parser::new(), code: VMCode::new(), data: VMData::new() }
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
        self.parser.load(src);
        self.parser.parse(&mut self.code);
    }
    pub fn log(&self) {
        for i in &self.code.interpreted {
            i.log();
        }
    }
    pub fn execute(&mut self) {
        while
            (
                self.data.compile &&
                self.data.ip < self.code.compiled.len()
            ) ||
            (
                !self.data.compile &&
                self.data.ip < self.code.interpreted.len()
            ) {
            if self.data.compile {
                self.code.compiled[self.data.ip].execute(&mut self.data);
            } else {
                self.code.interpreted[self.data.ip].execute(&mut self.data);
            }
            self.data.ip = self.data.ip + 1;
        }
    }
}
