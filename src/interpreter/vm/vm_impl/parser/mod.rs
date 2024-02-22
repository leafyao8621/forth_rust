use std::collections::HashMap;
use std::str;

mod handler;

use super::VMCode;
use self::handler::*;

pub struct ParserState {
    pub(in self) compile: bool,
    pub(in self) token: Vec<u8>,
    pub(in self) idx: usize,
    pub(in self) lookup:
        HashMap<Vec<u8>, Option<fn(&mut ParserState, &Vec<u8>, &mut VMCode)>>
}

impl ParserState {
    pub fn new() -> ParserState {
        ParserState {
            compile: false,
            token: Vec::new(),
            idx: 0,
            lookup: HashMap::new()
        }
    }
}

pub struct Parser {
    pub(in self) state: ParserState,
    pub(in self) src: Vec<u8>
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParserState::new(),
            src: Vec::new()
        }
    }
    pub fn load(&mut self, src: Vec<u8>) {
        self.src.truncate(0);
        self.src = src;
        self.state.lookup.insert(
            Vec::from(b".\""),
            Some(handle_print_string)
        );
        self.state.lookup.insert(
            Vec::from(b"cr"),
            Some(handle_cr)
        );
    }
    fn get_token(&mut self) {
        while self.state.idx < self.src.len() {
            if
                self.src[self.state.idx] == b' ' ||
                self.src[self.state.idx] == b'\t' ||
                self.src[self.state.idx] == b'\n' {
                break;
            }
            self.state.token.push(self.src[self.state.idx]);
            self.state.idx = self.state.idx + 1;
        }
    }
    fn next_token(&mut self) {
        while self.state.idx < self.src.len() {
            if
                self.src[self.state.idx] != b' ' &&
                self.src[self.state.idx] != b'\t' &&
                self.src[self.state.idx] != b'\n' {
                break;
            }
            self.state.idx = self.state.idx + 1;
        }
    }
    pub fn parse(&mut self, code: &mut VMCode) {
        while self.state.idx < self.src.len() {
            self.get_token();
            let s = match str::from_utf8(&self.state.token) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("{}", s);
            match self.state.lookup.get(&self.state.token) {
                Some(handler_ish) =>
                    match handler_ish {
                        Some(handler) =>
                            handler(&mut self.state, &self.src, code),
                        None => ()
                    },
                None => ()
            }
            self.state.token.truncate(0);
            self.next_token();
        }
    }
}
