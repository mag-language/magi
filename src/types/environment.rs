use crate::types::VariablePattern;
use crate::types::Obj;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub entries: HashMap<VariablePattern, Box<Obj>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn from(entries: HashMap<VariablePattern, Box<Obj>>) -> Self {
        Self {
            entries,
        }
    }

    pub fn extend(&self, other: Self) -> Self {
        let mut entries = self.entries.clone();
        entries.extend(other.entries);

        Self::from(entries)
    }
}