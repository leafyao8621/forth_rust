pub trait Instruction {
    fn execute(&self, machine: &mut VM);
}

pub struct VM {
    code: Vec<Box<Box<dyn Instruction>>>,
}

mod vm_impl;
