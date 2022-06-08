use crate::types::Obj;

/// An expression that evaluates to a value.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValuePattern {
    pub obj: Box<Obj>,
}