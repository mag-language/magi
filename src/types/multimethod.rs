use magc::{
    Expression,
    Pattern,
};

use std::collections::HashMap;

/// A method with a single name and many pairs of function signatures and bodies.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Multimethod {
    /// The individual signatures and bodies this multimethod is composed of.
    pub receivers: HashMap<Pattern, Expression>,
}