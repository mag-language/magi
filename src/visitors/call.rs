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
        expr: Call,
    ) -> InterpreterResult {
        let multimethod = interpreter.get_multimethod(&expr.name);

        Ok(Box::new(Expression {
            kind: ExpressionKind::Literal(Literal::Int),
            lexeme: format!(
                "{}", 
                42,
            ),
            start_pos: 0,
            end_pos: 0,
        }))
    }
}