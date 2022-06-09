use crate::types::*;
use crate::types::ObjKind::*;
use super::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::type_system::Typed;
use magc::types::{
    Expression,
    ExpressionKind,
    Conditional,
    Token,
    TokenKind,
};

pub struct ConditionalVisitor;

impl Visitor for ConditionalVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        let conditional = self::expect_conditional(obj)?;

        if interpreter.evaluate_expr(conditional.condition, optional_env.clone())?.is_truthy() {
            interpreter.evaluate_expr(conditional.then_arm, optional_env)
        } else {
            Ok(Box::new(Obj::new(ObjKind::Nothing)))
        }
    }
}

fn expect_conditional(obj: Obj) -> Result<Conditional, InterpreterError> {
    let found = obj.get_type();

    match obj.kind {
        ObjKind::Expression(expression) => {
            if let ExpressionKind::Conditional(conditional) = expression.kind {
                Ok(conditional)
            } else {
                Err(InterpreterError::UnexpectedType {
                    expected: format!("InfixExpression"),
                    found,
                })
            }
        },

        _ => Err(InterpreterError::UnexpectedType {
            expected: format!("ConditionalExpression"),
            found,
        }),
    }
}