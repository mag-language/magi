use magc::*;

use std::collections::HashMap;

pub struct Environment {
    entries: HashMap<VariablePattern, Box<Expression>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}