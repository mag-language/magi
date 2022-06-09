use crate::types::{Environment, Obj};
use crate::types::ObjKind::*;
use super::Visitor;

use crate::interpreter::{
    Interpreter,
    InterpreterResult,
    InterpreterError,
};

use magc::type_system::Typed;

pub struct ValueVisitor;

impl Visitor for ValueVisitor {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        optional_env: Option<Environment>,
        obj: Obj,
    ) -> InterpreterResult {

        /// Values simply evaluate to themselves.
        match &obj.kind {
            Int(_) | UInt(_) | Float(_) | String(_) | Boolean(_) => Ok(Box::new(obj)),

            _ => Err(InterpreterError::UnexpectedType { 
                found: obj.get_type(),
                expected: "Int | UInt | String | Float | Boolean".to_string(),
            })
        }
    }
}