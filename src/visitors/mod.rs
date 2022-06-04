mod call;

pub use self::call::*;

use crate::types::environment::Environment;
use crate::interpreter::{
    Interpreter,
    InterpreterResult,
};

/// A piece of code that knows how to evaluate a specific kind of expression.
pub trait Visitor<K> {
    fn parse(
        &self,
        interpreter: &mut Interpreter,
        environment_opt: Option<Environment>,
        expr: K,
    ) -> InterpreterResult;
}