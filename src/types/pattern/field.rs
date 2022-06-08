use super::Pattern;

/// A named pattern, like `repeats: 4` or `name: n String`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldPattern {
    pub name:  String,
    pub value: Box<Pattern>,
}