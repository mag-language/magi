use magc::{
    Expression,
};

use super::Multimethod;

pub enum Value {
    Multimethod(Multimethod),
    Expression(Expression),
}