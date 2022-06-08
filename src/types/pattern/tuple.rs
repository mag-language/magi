use super::Pattern;

/// A pattern enclosed in parentheses.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TuplePattern {
    pub child: Box<Pattern>,
}