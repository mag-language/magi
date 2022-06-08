use crate::types::{Environment, Obj, ObjKind};
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
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        let call = self::expect_call(obj)?;

        interpreter
            .get_multimethod(&call.name)?
            .call(call, optional_env)
    }
}

fn expect_call(obj: Obj) -> Result<Call, InterpreterError> {
    match obj.kind {
        ObjKind::Expression(expression) => {
            if let ExpressionKind::Call(call) = expression.kind {
                Ok(call)
            } else {
                Err(InterpreterError::UnexpectedType {
                    expected: String::from("CallExpression"),
                    found: expression.get_type(),
                })
            }
        },

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("CallExpression"),
            found: obj.get_type(),
        }),
    }
}