use crate::interpreter::vm::Instruction;
use crate::interpreter::VMData;

pub struct InstructionCR {
    repr: String
}

impl InstructionCR {
    pub fn new() -> InstructionCR {
        InstructionCR {
            repr: String::from("cr")
        }
    }
}

impl Instruction for InstructionCR {
    fn log(&self) {
        println!("Instruction: {}", self.repr);
    }
    fn execute(&self, _vm: &mut VMData) {
        print!("{}", '\n');
    }
}
