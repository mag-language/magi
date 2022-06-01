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

    /// Interpret a given piece of code.
    pub fn evaluate(
        &mut self,
        // The expression to evaluate.
        expression: Box<Expression>,
        // An optional environment used for variables in local scope.
        environment_opt: Option<Environment>,
    ) -> Result<Box<Expression>, InterpreterError> {
        match expression.kind {
            ExpressionKind::Method(method) => self.define_method(method),
            ExpressionKind::Call(call)     => self.call_method(call, environment_opt),
            //ExpressionKind::Infix(infix)   => self.evaluate_infix(infix),

            _ => Err(InterpreterError::Unimplemented),
        }
    }

    fn define_method(
        &mut self,
        method: Method,
    ) -> Result<Box<Expression>, InterpreterError> {
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

    fn call_method(
        &mut self,
        call: Call,
        // An optional environment used for variables in local scope.
        environment_opt: Option<Environment>,
    ) -> Result<Box<Expression>, InterpreterError> {
        // This array contains the matching patterns (if any) which will then be sorted by precedence.
        let mut matching_receivers =  vec![];

        if let Some(multimethod) = self.methods.get(&call.name) {
            // Iterate over each of the possibly matching methods.
            for (sig, body) in &multimethod.receivers {
                if let Some(ref signature) = call.signature {
                    // Compare the method signature to the reference.
                    if let Ok(extracted_variables) = sig.linearize(signature.clone()) {
                        matching_receivers.push((signature.clone(), body.clone(), extracted_variables));
                    }
                }
            }

            // Sort the matching receivers so that the most specific pattern is first in line.
            matching_receivers.sort_by(|a, b| b.0.get_precedence().cmp(&a.0.get_precedence()));

            if matching_receivers.len() > 0 {
                let (sig, body, vars) = matching_receivers[0].clone();
                println!("matching_receiver: {:#?}\n\n{:#?}", body, vars);

                // check if single expression, in this case evaluate and return
                //
                // otherwise it's a block, iterate and evaluate each of the expressions
                // using the extracted variables as the function scope.
            } else {
                return Err(InterpreterError::NoMatchingReceiver)
            }

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

#[derive(Debug)]
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