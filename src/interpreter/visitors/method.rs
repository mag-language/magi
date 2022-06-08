use crate::types::{Environment, Obj, ObjKind, Pattern, VariablePattern};
use crate::types::Multimethod;
use super::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::types::{Method, Expression, ExpressionKind, Literal};
use magc::types::Pattern as MagcPattern;
use magc::type_system::Typed;

pub struct MethodVisitor;

impl Visitor for MethodVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        let method = self::expect_method(obj)?;
        let name   = VariablePattern { name: Some(method.name.clone()), type_id: None };

        match interpreter.get_variable(name) {
            // There is already a multimethod with this name, so try to insert the new receiver.
            Ok(obj) => {
                let mut multimethod = self::expect_multimethod(*obj)?;

                // Add the new receiver to the multimethod.
                multimethod.define(
                    self::pattern_or_none(method.signature),
                    Box::new(Obj::from(*method.body)),
                )?;
            },

            // There is no multimethod definition with the given name, so create a new one.
            Err(InterpreterError::NoMatchingVariable) => {

                let multimethod = Multimethod::from(
                    self::pattern_or_none(method.signature),
                    Box::new(Obj::from(*method.body)),
                );

                // Create a new multimethod with the given receiver and register it in the interpreter.
                interpreter.methods.insert(
                    method.name.clone(),
                    multimethod,
                );
            },

            Err(e) => return Err(e),
        }

        Ok(Box::new(
            Obj::new(ObjKind::Type("Method".to_string()))
        ))
    }
}

fn pattern_or_none(pattern: Option<MagcPattern>) -> Option<Pattern> {
    if let Some(p) = pattern {
        Some(Pattern::from(p))
    } else {
        None
    }
}

fn expect_method(obj: Obj) -> Result<Method, InterpreterError> {
    let found = obj.get_type();

    match obj.kind {
        ObjKind::Expression(expression) => {
            if let ExpressionKind::Method(method) = expression.kind {
                Ok(method)
            } else {
                Err(InterpreterError::UnexpectedType {
                    expected: String::from("MethodExpression"),
                    found,
                })
            }
        },

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("MethodExpression"),
            found,
        }),
    }
}

fn expect_multimethod(obj: Obj) -> Result<Multimethod, InterpreterError> {
    match obj.kind {
        ObjKind::Multimethod(m) => Ok(m),

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("Multimethod"),
            found: obj.get_type(),
        }),
    }
}