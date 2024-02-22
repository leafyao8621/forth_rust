use crate::interpreter::vm::{
    vm_impl::{
        instruction::InstructionCR, parser::ParserState
    },
    VMCode
};

pub fn handle_cr(
    state: &mut ParserState, _src: &Vec<u8>, code: &mut VMCode) {
    if state.compile {
        code.compiled.push(
            Box::new(
                InstructionCR::new()
            )
        );
    } else {
        code.interpreted.push(
            Box::new(
                InstructionCR::new()
            )
        );
    }
}
