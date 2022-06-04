use magc::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    pub entries: HashMap<VariablePattern, Box<Expression>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn from(map: HashMap<VariablePattern, Box<Expression>>) -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}