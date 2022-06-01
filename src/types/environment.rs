use magc::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
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