use magc::{
    Expression,
    Method,
    Pattern,
};

use magc::parser::{
    ParserResult,
    ParserError,
};

use std::collections::BTreeMap;

/// A method with a single name and many pairs of function signatures and bodies.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Multimethod {
    /// The individual signatures and bodies this multimethod is composed of.
    pub receivers: BTreeMap<Pattern, Expression>,
}

impl Multimethod {
    /// Create a new multimethod from the given [`Method`].
    pub fn from(method: Method) -> Self {
        Self {
            receivers: BTreeMap::new(),
        }
    }

    // / Try to match the given call signature with one of the method implementations.
    pub fn call(&self, signature: Pattern) -> Result<(), ParserError> {
        for (pattern, body) in &self.receivers {
            match signature.linearize(pattern.clone()) {
                Ok(variables) => {
                    println!("[✓] This is a match");
                    println!("Extracted variables: {:?}", variables);
                },

                Err(e) => {
                    println!("[⨯] This is no match");
                    return Err(ParserError::NoMatch)
                }
            }
        }

        Ok(())
    }
}