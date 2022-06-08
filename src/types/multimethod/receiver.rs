use crate::types::{Obj, Pattern};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Receiver {
    pub signature: Option<Pattern>,
    pub body:      Box<Obj>,
}

impl Receiver {
    pub fn from(signature: Option<Pattern>, body: Box<Obj>) -> Self {
        Self {
            signature,
            body,
        }
    }
}