use super::VM;
mod parser;

impl VM {
    pub fn new(file_name: &str) -> VM {
        println!("{}", file_name);
        VM { code: Vec::new() }
    }
}
