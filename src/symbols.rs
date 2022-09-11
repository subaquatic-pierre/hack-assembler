use std::collections::HashMap;

use crate::utils::{build_built_in, build_comp, build_dest, build_jump};

#[derive(Debug)]
pub struct Symbols {
    pub built_in: HashMap<String, i32>,
    pub label: HashMap<String, i32>,
    pub variable: HashMap<String, i32>,
    pub comp: HashMap<String, String>,
    pub dest: HashMap<String, String>,
    pub jump: HashMap<String, String>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            built_in: build_built_in(),
            label: HashMap::new(),
            variable: HashMap::new(),
            comp: build_comp(),
            dest: build_dest(),
            jump: build_jump(),
        }
    }
}
