pub mod receiver;

pub use self::receiver::Receiver;

use magc::types::{
    Expression,
    ExpressionKind,
    Call,
};

use magc::types::Pattern as MagcPattern;

use crate::types::{
    Obj,
    ObjKind,
    Pattern,
};

use crate::interpreter::Interpreter;

use crate::interpreter::InterpreterError;
use super::Environment;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Multimethod {
    pub receivers: Vec<Receiver>,
}

impl Multimethod {
    pub fn from(signature: Option<Pattern>, body: Box<Obj>) -> Self {
        let mut receivers = vec![];

        receivers.push(Receiver::from(signature, body));

        Self {
            receivers,
        }
    }

    /// Add a new receiver to this multimethod if it does not already exist.
    pub fn define(&mut self, signature: Option<Pattern>, body: Box<Obj>) -> Result<(), InterpreterError> {
        if let None = self.receivers.iter().find(|recv| recv.signature == signature) {
            self.receivers.push(Receiver::from(signature, body));

            Ok(())
        } else {
            Err(InterpreterError::MethodAlreadyExists)
        }
    }

    /// Try to find a matching receiver, run its body with the bound variables and return a value, if any.
    pub fn call(&self, interpreter: &mut Interpreter, signature: Option<MagcPattern>) -> Result<Box<Obj>, InterpreterError> {
        let evaluated_signature;

        if let Some(magc_pattern) = signature {
            let obj = interpreter.evaluate(
                Box::new(
                    Obj::new(ObjKind::Pattern(Pattern::from(magc_pattern)))
                ),
                None,
            )?;

            evaluated_signature = Some(self::expect_pattern(*obj)?);
        } else {
            evaluated_signature = None;
        };

        // Find matching receivers and sort them so the one with the highest precedence value goes first.
        let mut matching_receivers = self.find_matching_receivers(evaluated_signature)?;

        matching_receivers.sort_by(|a, b| b.2.cmp(&a.2));

        if matching_receivers.len() >= 1 {
            let (env, obj, _) = matching_receivers[0].clone();

            interpreter.evaluate(
                obj,
                Some(env),
            )
        } else {
            return Err(InterpreterError::NoMatchingReceiver)
        }
    }

    fn find_matching_receivers(&self,
        reference_sig: Option<Pattern>,
    ) -> Result<Vec<(Environment, Box<Obj>, usize)>, InterpreterError> {

        self.receivers
            .iter()
            // Filter out any receivers which don't have a matching signature.
            .filter(|recv| {
                self::match_pattern(
                    if let Some(s) = &recv.signature { Some(Pattern::from(s.clone())) } else { None },
                    if let Some(s) = &reference_sig  { Some(Pattern::from(s.clone())) } else { None },
                )
            })
            // Convert the matching receivers to a tuple containing the extracted variables,
            // the body expression and the pattern's precedence to simplify sorting later.
            .map(|recv| {
                Ok((
                    // Extract the variables which will be bound to function scope.
                    self::match_pattern_and_extract(
                        if let Some(s) = &recv.signature { Some(Pattern::from(s.clone())) } else { None },
                        if let Some(s) = &reference_sig  { Some(Pattern::from(s.clone())) } else { None },
                    ),
                    recv.body.clone(),
                    self::get_precedence(
                        if let Some(s) = &reference_sig { Some(Pattern::from(s.clone())) } else { None },
                    ),
                ))
            })
            .collect()
    }
}

fn get_precedence(item: Option<Pattern>) -> usize {
    match item {
        Some(pattern) => pattern.get_precedence(),
        None          => 0,
    }
}

fn expect_pattern(obj: Obj) -> Result<Pattern, InterpreterError> {
    match obj.kind {
        ObjKind::Pattern(pattern) => Ok(pattern),

        // TODO: do correct error reporting here.
        _ => return Err(InterpreterError::NoMatch)
    }
}

fn match_pattern(reference: Option<Pattern>, given: Option<Pattern>) -> bool {
    match (reference, given) {
        (None, None) => true,
        (Some(r), Some(g)) => r.matches_with(g),

        _ => false,
    }
}

fn match_pattern_and_extract(reference: Option<Pattern>, given: Option<Pattern>) -> Environment {
    match (reference, given) {
        (None, None) => Environment::empty(),

        (Some(r), Some(g)) => r.linearize(g).unwrap(),

        _ => Environment::empty(),
    }
}