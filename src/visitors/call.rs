use crate::types::environment::Environment;
use crate::visitors::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
};

use magc::types::{Call, Expression, ExpressionKind, Literal};

pub struct CallVisitor;

impl Visitor<Call> for CallVisitor {
    fn parse(
        &self,
        interpreter: &mut Interpreter,
        environment_opt: Option<Environment>,
        expr: Call,
    ) -> InterpreterResult {

        interpreter.get_multimethod(&expr.name)?
            .call(expr, environment_opt)
    }
}