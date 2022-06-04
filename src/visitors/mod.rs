mod call;

pub use self::call::*;
use crate::interpreter::{
    Interpreter,
    InterpreterResult,
};

/// A piece of code that knows how to evaluate a specific kind of expression.
pub trait Visitor<K> {
    fn parse(
        &self,
        interpreter: &mut Interpreter,
        kind: K,
    ) -> InterpreterResult;
}