use crate::types::environment::Environment;
use crate::interpreter::Multimethod;
use crate::visitors::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::types::{Method, Expression, ExpressionKind, Literal};
use magc::type_system::Typed;

pub struct MethodVisitor;

impl Visitor for MethodVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        environment_opt: Option<Environment>,
        expression: Expression,
    ) -> InterpreterResult {

        let method = self::expect_method(expression)?;

        if let Some(multimethod) = interpreter.methods.get_mut(&method.name) {
            // Add the new receiver to the multimethod.
            multimethod.define(method.signature, method.body);
        } else {
            // Create a new multimethod with the given receiver and register it in the interpreter.
            interpreter.methods.insert(
                method.name.clone(),
                Multimethod::from(method.signature, method.body)
            );
        }
        Ok(Box::new(
            Expression {
                kind: ExpressionKind::Identifier,
                start_pos: 0,
                end_pos: 0,
                lexeme: method.name,
            }
        ))
    }
}

fn expect_method(expression: Expression) -> Result<Method, InterpreterError> {
    match expression.kind {
        ExpressionKind::Method(method) => Ok(method),

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("CallExpression"),
            found: expression.get_type(),
        }),
    }
}