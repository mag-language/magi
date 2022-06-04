pub mod environment;
pub mod obj;
pub mod multimethod;

pub use self::environment::Environment;
pub use self::multimethod::*;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
};

use magc::types::Expression;

/// A piece of code that can evaluate a specific kind of expression.
pub trait Visitor {
    fn parse(&self, interpreter: &mut Interpreter, expr: Box<Expression>) -> InterpreterResult;
}
