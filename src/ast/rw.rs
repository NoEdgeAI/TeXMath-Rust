use super::{node, tex_writer::ToTeX, ast_reader};
use std::{collections::HashMap, thread::panicking};
use nom::IResult;
// Exp read and write
pub struct ExpRW {
    input: String,
    e : node::Exp,
    valid: bool,
    envs: HashMap<String, bool>,
}


impl ExpRW {
    // from a envs to create a parser
    pub fn new(envs: HashMap<String, bool>) -> Self {
        ExpRW {
            input: String::new(),
            e: node::Exp::ENumber("0".to_string()),
            envs: envs,
            valid: false,
        }
    }
    
    // read from ast
    pub fn from_ast(&mut self, ast: &str) -> Result<(), String> {
        if self.valid {
            return Result::Err("Already read from ast".to_string());
        }

        self.input = ast.to_string();
        match ast_reader::parse_exp(ast) {
            Ok((_, e)) => {
                self.e = e;
                self.valid = true;
                Ok(())
            },
            Err(e) => {
                let msg = format!("Parse error: {:?}", e);
                Result::Err(msg)
            }
        }
    }
}
