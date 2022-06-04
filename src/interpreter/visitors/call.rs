use crate::types::environment::Environment;
use super::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::types::{Call, Expression, ExpressionKind, Literal};
use magc::type_system::Typed;

pub struct CallVisitor;

impl Visitor for CallVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        environment_opt: Option<Environment>,
        expression: Expression,
    ) -> InterpreterResult {

        let call = self::expect_call(expression)?;

        interpreter
            .get_multimethod(&call.name)?
            .call(call, environment_opt)
    }
}

fn expect_call(expression: Expression) -> Result<Call, InterpreterError> {
    match expression.kind {
        ExpressionKind::Call(call) => Ok(call),

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("CallExpression"),
            found: expression.get_type(),
        }),
    }
}