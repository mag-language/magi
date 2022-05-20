use magc::*;
use super::Value;

use std::collections::HashMap;

pub struct Environment {
    entries: HashMap<VariablePattern, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}