use crate::types::{Environment, Obj, ObjKind, VariablePattern, Multimethod};
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

        let variable = interpreter.get_variable(VariablePattern::from_name(call.name.clone()))?;

        self::expect_multimethod(*variable)?
            .call(interpreter, call)
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

fn expect_multimethod(obj: Obj) -> Result<Multimethod, InterpreterError> {
    match obj.kind {
        ObjKind::Multimethod(multimethod) => Ok(multimethod),

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("Multimethod"),
            found: obj.get_type(),
        }),
    }
}