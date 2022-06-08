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
    Infix,
    Token,
    TokenKind,
};

pub struct InfixVisitor;

impl Visitor for InfixVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        let infix = self::expect_infix_expression(obj)?;
        
        let left = interpreter.evaluate_expr(infix.left, optional_env.clone())?;
        let right = interpreter.evaluate_expr(infix.right, optional_env)?;

        match infix.operator.kind {
            TokenKind::Plus  => Ok(Box::new((*left + *right)?)),
            TokenKind::Minus => Ok(Box::new((*left - *right)?)),
            TokenKind::Slash => Ok(Box::new((*left / *right)?)),
            TokenKind::Star  => Ok(Box::new((*left * *right)?)),

            _ => Err(InterpreterError::Unimplemented),
        }
    }
}

fn expect_infix_expression(obj: Obj) -> Result<Infix, InterpreterError> {
    let found = obj.get_type();

    match obj.kind {
        ObjKind::Expression(expression) => {
            if let ExpressionKind::Infix(method) = expression.kind {
                Ok(method)
            } else {
                Err(InterpreterError::UnexpectedType {
                    expected: format!("InfixExpression"),
                    found,
                })
            }
        },

        _ => Err(InterpreterError::UnexpectedType {
            expected: format!("InfixExpression"),
            found,
        }),
    }
}