use crate::types::{
    Environment,
    Obj,
    ObjKind,
    Pattern,
    FieldPattern,
    PairPattern,
    TuplePattern,
    ValuePattern,
    VariablePattern,
};
use crate::types::Multimethod;
use super::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::types::{Method, Expression, ExpressionKind, Literal};
use magc::type_system::Typed;

pub struct PatternVisitor;

impl Visitor for PatternVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        let pattern = self::expect_pattern(obj)?;

        match pattern {
            Pattern::Variable(variable_pattern) => {

            },

            _ => Ok(Box::new(Obj::new(ObjKind::Pattern(pattern))))
        }

        Ok(Box::new(
            Obj::new(ObjKind::Type("Method".to_string()))
        ))
    }
}

fn expect_pattern(obj: Obj) -> Result<Pattern, InterpreterError> {
    match obj.kind {
        ObjKind::Pattern(pattern) => Ok(pattern),

        _ => Err(InterpreterError::UnexpectedType {
            expected: String::from("Pattern"),
            found: obj.get_type(),
        }),
    }
}