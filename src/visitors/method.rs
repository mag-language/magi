use crate::types::environment::Environment;
use crate::visitors::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::types::{Method, Expression, ExpressionKind, Literal};
use magc::type_system::Typed;

pub struct CallVisitor;

impl Visitor for CallVisitor {
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
            // Create a new multimethod and insert the new receiver.
            let multimethod = Multimethod::from(method.signature, method.body);
            interpreter.methods.insert(method.name.clone(), multimethod);
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