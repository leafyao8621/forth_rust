use self::vm_impl::parser::Parser;

mod vm_impl;

pub struct VMData {
    pub(in self) ip: usize,
    pub(in self) compile: bool,
    pub(in self) main_stack: Vec<i64>,
    pub(in self) control_stack: Vec<i64>,
}

pub trait Instruction {
    fn log(&self);
    fn execute(&self, data: &mut VMData);
}

pub struct VMCode {
    pub(in self) compiled: Vec<Box<dyn Instruction>>,
    pub(in self) interpreted: Vec<Box<dyn Instruction>>,
}

pub struct VM {
    parser: Parser,
    pub(in self) code: VMCode,
    pub(in self) data: VMData,
}
