use magc::*;

use std::collections::HashMap;

pub struct Environment {
    entries: HashMap<VariablePattern, Expression>,
}