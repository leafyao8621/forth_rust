use crate::interpreter::vm::Instruction;
use crate::interpreter::VMData;

pub struct InstructionPrintString {
    repr: String,
    data: String
}

impl InstructionPrintString {
    pub fn new(data: &str) -> InstructionPrintString {
        InstructionPrintString {
            repr: String::from(".\""),
            data: String::from(data)
        }
    }
}

impl Instruction for InstructionPrintString {
    fn log(&self) {
        println!("Instruction: {}", self.repr);
        println!("Data: {}", self.data);
    }
    fn execute(&self, _vm: &mut VMData) {
        print!("{}", self.data);
    }
}
