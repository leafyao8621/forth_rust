use crate::interpreter::VM;
use crate::interpreter::vm::vm_impl::instruction::InstructionPrintString;

pub struct Parser {
    src: Vec<u8>
}

impl Parser {
    pub fn new(src: Vec<u8>) -> Parser {
        Parser { src:  src }
    }
    pub fn parse(&mut self, vm: &mut VM) {
        vm.code.push(Box::new(InstructionPrintString::new("abcd")));
    }
}
