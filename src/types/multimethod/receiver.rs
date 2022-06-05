use magc::types::{
    Expression,
    Pattern,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub signature: Option<Pattern>,
    pub body:      Box<Expression>,
}

impl Receiver {
    pub fn from(signature: Option<Pattern>, body: Box<Expression>) -> Self {
        Self {
            signature,
            body,
        }
    }
}