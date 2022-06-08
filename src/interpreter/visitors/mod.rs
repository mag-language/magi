mod call;
mod method;

pub use self::call::*;
pub use self::method::*;

use crate::types::{Environment, Obj};
use crate::interpreter::{
    Interpreter,
    InterpreterResult,
};
use magc::types::Expression;

/// A piece of code that knows how to evaluate a specific kind of expression.
pub trait Visitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult;
}