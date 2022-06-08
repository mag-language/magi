use super::Pattern;

/// A pair of patterns separated by a comma.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PairPattern {
    pub left: Box<Pattern>,
    pub right: Box<Pattern>,
}