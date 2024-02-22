use crate::interpreter::vm::{
    vm_impl::{
        instruction::InstructionPrintString, parser::ParserState
    },
    VMCode
};

pub fn handle_print_string(
    state: &mut ParserState, src: &Vec<u8>, code: &mut VMCode) {
    state.token.truncate(0);
    state.idx = state.idx + 1;
    while state.idx < src.len() && src[state.idx] != b'"' {
        state.token.push(src[state.idx]);
        state.idx = state.idx + 1;
    }
    state.idx = state.idx + 1;
    if state.compile {
        code.compiled.push(
            Box::new(
                InstructionPrintString::new(&state.token)
            )
        );
    } else {
        code.interpreted.push(
            Box::new(
                InstructionPrintString::new(&state.token)
            )
        );
    }
}
