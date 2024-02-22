mod vm_impl;

pub struct VMData {
    pub(in crate::interpreter::vm) main_stack: Vec<i64>,
    pub(in crate::interpreter::vm) control_stack: Vec<i64>,
}

pub trait Instruction {
    fn log(&self);
    fn execute(&self, vm: &mut VMData);
}


pub struct VM {
    pub(in crate::interpreter::vm) code: Vec<Box<dyn Instruction>>,
    pub(in crate::interpreter::vm) data: VMData,
}
