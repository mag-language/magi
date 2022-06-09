mod call;
mod conditional;
mod method;
mod value;
mod infix;
mod pattern;

pub use self::call::*;
pub use self::conditional::*;
pub use self::method::*;
pub use self::value::*;
pub use self::infix::*;
pub use self::pattern::*;

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