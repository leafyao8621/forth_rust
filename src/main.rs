mod interpreter;

fn main() {
    let mut vm = interpreter::VM::new();
    vm.load("scripts/a.fs");
    vm.log();
    vm.execute();
}
