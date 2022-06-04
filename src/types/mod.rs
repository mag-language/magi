pub mod environment;
pub mod obj;

pub use self::environment::*;

/// A piece of code that can evaluate a specific kind of expression.
pub trait Visitor {
    fn parse(&self, interpreter: &mut Interpreter, expr: Box<Expression>) -> InterpreterResult;
}
