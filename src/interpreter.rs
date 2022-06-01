use std::collections::HashMap;

use crate::types::environment::Environment;

use magc::types::{
    Expression,
    ExpressionKind,
    Method,
    Call,
    Infix,
    Block,
    Pattern,
    PairPattern,
    ValuePattern,
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
            ExpressionKind::Literal(_)     => { return Ok(expression) },
            ExpressionKind::List(_)        => { return Ok(expression) },
            ExpressionKind::Type           => { return Ok(expression) },
            ExpressionKind::Infix(infix)   => self.evaluate_infix(infix, environment_opt),

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

                // Evaluate a single expression or the children of a block.
                match body.kind {
                    ExpressionKind::Block(mut block) => {
                        let last_child = block.children.pop();

                        for expr in block.children {
                            self.evaluate(Box::new(expr.clone()), environment_opt.clone());
                        }

                        return Err(InterpreterError::Unimplemented)
                    },

                    _ => {
                        return self.evaluate(body, environment_opt.clone())
                    },
                }
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

    fn pattern_or_value_pattern(&self, expression: Box<Expression>) -> Pattern {
        match expression.kind {
            ExpressionKind::Pattern(pattern) => pattern,

            _ => Pattern::Value(ValuePattern {
                expression,
            }),
        }
    }

    fn evaluate_infix(
        &mut self,
        infix: Infix,
        // An optional environment used for variables in local scope.
        environment_opt: Option<Environment>,
    ) -> Result<Box<Expression>, InterpreterError> {
        let left = self.evaluate(infix.left.clone(), environment_opt.clone())?.clone();
        let right = self.evaluate(infix.right.clone(), environment_opt.clone())?.clone();

        let signature = Some(Pattern::Pair(
            PairPattern {
                left:  Box::new(self.pattern_or_value_pattern(left)),
                right: Box::new(self.pattern_or_value_pattern(right)),
            }
        ));

        self.call_method(
            Call {
                name: infix.operator.lexeme,
                signature,
            },
            environment_opt,
        )
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