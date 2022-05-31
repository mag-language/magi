use std::collections::HashMap;

use crate::types::{Environment};

use magc::types::{
    Expression,
    ExpressionKind,
    Method,
    Call,
    Block,
    Pattern,
};

pub struct Interpreter {
    environment: Environment,
    methods: HashMap<String, Multimethod>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            methods:     HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, expression: Box<Expression>) -> Result<Box<Expression>, InterpreterError> {
        match expression.kind {
            ExpressionKind::Method(method) => self.define_method(method),
            ExpressionKind::Call(call)     => self.call_method(call),

            _ => Err(InterpreterError::Unimplemented),
        }
    }

    fn define_method(&mut self, method: Method) -> Result<Box<Expression>, InterpreterError> {
        if let Some(multimethod) = self.methods.get_mut(&method.name) {
            multimethod.define(method.signature.unwrap(), method.body);
        } else {
            let m = Multimethod::from(method.signature.unwrap(), method.body);
            self.methods.insert(method.name.clone(), m);
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

    fn call_method(&mut self, call: Call) -> Result<Box<Expression>, InterpreterError> {
        let mut does_match = false;

        if let Some(multimethod) = self.methods.get(&call.name) {
            for (sig, body) in &multimethod.receivers {
                if let Some(ref signature) = call.signature {
                    match sig.linearize(signature.clone()) {
                        Ok(extracted_variables) => {
                            does_match = true;
                            println!("MATCH! linearizing: {:?} with {:?}", sig, signature);
                        },

                        Err(e) => {
                            println!("no match linearizing: {:?} with {:?}", sig, signature);
                        },
                    }
                }
            }

            println!("does_match = {}", does_match);

            Ok(Box::new(
                Expression {
                    kind: ExpressionKind::Identifier,
                    start_pos: 0,
                    end_pos: 0,
                    lexeme: call.name,
                }
            ))
        } else {
            Err(InterpreterError::NoMatchingMultimethod)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum InterpreterError {
    Unimplemented,
    MethodAlreadyExists,
    SignatureAlreadyExists,
    NoMatchingReceiver,
    NoMatchingMultimethod,
}

pub struct Multimethod {
    pub receivers: HashMap<Pattern, Box<Expression>>,
}

impl Multimethod {
    pub fn from(signature: Pattern, body: Box<Expression>) -> Self {
        let mut receivers = HashMap::new();

        receivers.insert(signature, body);

        Self {
            receivers,
        }
    }

    /// Add a new receiver to this multimethod if it does not already exist.
    pub fn define(&mut self, signature: Pattern, body: Box<Expression>) -> Result<(), InterpreterError> {
        if !self.receivers.contains_key(&signature) {
            self.receivers.insert(signature, body);

            Ok(())
        } else {
            Err(InterpreterError::MethodAlreadyExists)
        }
    }

    pub fn call(&self, given_signature: Pattern) -> Result<(), InterpreterError> {
        for (sig, body) in &self.receivers {
            println!("{:?}", sig);
        }

        Ok(())
    }
}