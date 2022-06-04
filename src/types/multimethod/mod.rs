pub mod receiver;

pub use self::receiver::Receiver;

use magc::types::{
    Expression,
    ExpressionKind,
    Pattern,
    Call,

};

use crate::interpreter::InterpreterError;
use super::Environment;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Multimethod {
    pub receivers: Vec<Receiver>,
}

impl Multimethod {
    pub fn from(signature: Option<Pattern>, body: Box<Expression>) -> Self {
        let mut receivers = vec![];

        receivers.push(Receiver::from(signature, body));

        Self {
            receivers,
        }
    }

    /// Add a new receiver to this multimethod if it does not already exist.
    pub fn define(&mut self, signature: Option<Pattern>, body: Box<Expression>) -> Result<(), InterpreterError> {
        if let None = self.receivers.iter().find(|recv| recv.signature == signature) {
            self.receivers.push(Receiver::from(signature, body));

            Ok(())
        } else {
            Err(InterpreterError::MethodAlreadyExists)
        }
    }

    /// Try to find a matching receiver, run its body with the bound variables and return a value, if any.
    pub fn call(&self, call: Call, environment_opt: Option<Environment>) -> Result<Box<Expression>, InterpreterError> {
        // Find matching receivers and sort them so the one with the highest precedence value goes first.
        let mut matching_receivers = self.find_matching_receivers(call.clone(), environment_opt)?;
        matching_receivers.sort_by(|a, b| b.2.cmp(&a.2));

        if matching_receivers.len() >= 1 {
            println!("{:#?}", matching_receivers.first());
        } else {
            return Err(InterpreterError::NoMatchingReceiver)
        }

        Ok(Box::new(Expression {
            kind: ExpressionKind::Identifier,
            lexeme: call.name,
            start_pos: 0,
            end_pos: 0,
        }))
    }

    fn find_matching_receivers(&self,
        call: Call,
        environment_opt: Option<Environment>,
    ) -> Result<Vec<(Environment, Box<Expression>, usize)>, InterpreterError> {

        self.receivers
            .iter()
            // Filter out any receivers which don't have a matching signature.
            .filter(|recv| {
                !self::match_pattern(
                    recv.signature.clone(),
                    call.signature.clone(),
                )
            })
            // Convert the matching receivers to a tuple containing the extracted variables,
            // the body expression and the pattern's precedence to simplify sorting later.
            .map(|recv| {
                Ok((
                    // Extract the variables which will be bound to function scope.
                    self::match_pattern_and_extract(
                        recv.signature.clone(),
                        call.signature.clone()
                    ),
                    recv.body.clone(),
                    self::get_precedence(call.signature.clone()),
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

fn match_pattern(reference: Option<Pattern>, given: Option<Pattern>) -> bool {
    match (reference, given) {
        (None, None) => true,
        (Some(r), Some(g)) => r.matches_with(g),

        _ => false,
    }
}

fn match_pattern_and_extract(reference: Option<Pattern>, given: Option<Pattern>) -> Environment {
    let map = match (reference, given) {
        (None, None) => HashMap::new(),

        (Some(r), Some(g)) => r.linearize(g).unwrap(),

        _ => HashMap::new(),
    };

    Environment::from(map)
}