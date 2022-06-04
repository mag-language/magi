use std::collections::{HashMap, BTreeMap};

use crate::types::environment::Environment;

use crate::visitors::Visitor;

use magc::type_system::Typed;

use magc::types::{
    Literal,
    Expression,
    ExpressionKind,
    Method,
    Call,
    Infix,
    Block,
    Pattern,
    PairPattern,
    ValuePattern,
    VariablePattern,
};

pub type InterpreterResult = Result<Box<Expression>, InterpreterError>;

pub struct Interpreter {
    environment: Environment,
    methods: HashMap<String, Multimethod>,
    visitors: HashMap<String, &'static dyn Visitor>,
    recursion_level: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            methods:     HashMap::new(),
            visitors:    HashMap::new(),
            recursion_level: 0,
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
        println!("evaluating: --- {:#?}", expression.kind);

        match expression.kind {
            ExpressionKind::Method(method) => self.define_method(method),
            ExpressionKind::Call(call)     => self.call_method(call, environment_opt),
            ExpressionKind::Literal(_)     => { return Ok(expression) },
            ExpressionKind::List(_)        => { return Ok(expression) },
            ExpressionKind::Type           => { return Ok(expression) },
            ExpressionKind::Infix(infix)   => self.evaluate_infix(infix, environment_opt),
            ExpressionKind::Pattern(pattern) => Ok(Box::new(Expression {
                // Unwrap is safe here because there is always a pattern in the expression.
                kind: ExpressionKind::Pattern(self.evaluate_pattern(Some(pattern), environment_opt)?.unwrap()),
                start_pos: expression.start_pos,
                end_pos: expression.end_pos,
                lexeme: expression.lexeme,
            })),

            _ => Err(InterpreterError::Unimplemented),
        }
    }

    fn define_method(
        &mut self,
        method: Method,
    ) -> Result<Box<Expression>, InterpreterError> {
        if let Some(multimethod) = self.methods.get_mut(&method.name) {
            multimethod.define(method.signature, method.body);
        } else {
            let m = Multimethod::from(method.signature, method.body);
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

    fn do_arithmetic(&self, call: Call) -> Result<Box<Expression>, InterpreterError> {
        if let Some(signature) = call.signature {
            let pair = self.expect_pair(Box::new(signature))?;

            let num = match call.name.as_str() {
                "+" => self.expect_int(pair.left)? + self.expect_int(pair.right)?,
                "-" => self.expect_int(pair.left)? - self.expect_int(pair.right)?,
                "*" => self.expect_int(pair.left)? * self.expect_int(pair.right)?,
                "/" => self.expect_int(pair.left)? / self.expect_int(pair.right)?,
    
                _ => unreachable!(),
            };

            Ok(Box::new(Expression {
                kind: ExpressionKind::Literal(Literal::Int),
                lexeme: format!(
                    "{}", 
                    num,
                ),
                start_pos: 0,
                end_pos: 0,
            }))
        } else {
            Err(InterpreterError::Unimplemented)
        }
    }

    fn expect_pair(&self, pattern: Box<Pattern>) -> Result<PairPattern, InterpreterError> {
        match *pattern {
            Pattern::Pair(pair) => Ok(pair),

            _ => Err(
                InterpreterError::UnexpectedType { 
                    expected: "PairPattern".to_string(),
                    found: pattern.get_type(),
                }
            )
        }
    }

    fn expect_int(&self, pattern: Box<Pattern>) -> Result<i64, InterpreterError> {
        match *pattern {
            Pattern::Value(ValuePattern { expression }) => {
                match expression.kind {
                    ExpressionKind::Literal(Literal::Int) => {
                        Ok(expression.lexeme.parse::<i64>().unwrap())
                    },

                    _ => Err(
                        InterpreterError::UnexpectedType { 
                            expected: "PairPattern".to_string(),
                            found: None,
                        }
                    ),
                }
            },

            _ => Err(
                InterpreterError::UnexpectedType { 
                    expected: "Int".to_string(),
                    found: None,
                }
            )
        }
    }

    fn evaluate_pattern(
        &mut self,
        pattern: Option<Pattern>,
        environment_opt: Option<Environment>,
    ) -> Result<Option<Pattern>, InterpreterError> {
        match pattern {
            None => Ok(None),

            Some(Pattern::Value(value_pattern)) => Ok(
                Some(Pattern::Value(ValuePattern {
                    expression: self.evaluate(value_pattern.expression.clone(), environment_opt)?,
                }))
            ),

            Some(Pattern::Variable(variable_pattern)) => {
                if let Some(env) = environment_opt {
                    if let Some(expr) = env.entries.get(&variable_pattern) {
                        Ok(Some(Pattern::Value(ValuePattern {
                            expression: expr.clone(),
                        })))
                    } else {
                        Err(InterpreterError::NoMatchingVariable)
                    }
                } else {
                    if let Some(expr) = self.environment.entries.get(&variable_pattern) {
                        Ok(Some(Pattern::Value(ValuePattern {
                            expression: expr.clone(),
                        })))
                    } else {
                        Err(InterpreterError::NoMatchingVariable)
                    }
                }
            },

            _ => Ok(pattern),
        }
    }

    pub fn get_multimethod(&self, name: &String) -> Result<Multimethod, InterpreterError> {
        if let Some(multimethod) = self.methods.get(name) {
            Ok(multimethod.clone())
        } else {
            Err(InterpreterError::NoMatchingMultimethod)
        }
    }

    /**fn sort_receivers(&self,
        receivers: HashMap<Option<Pattern>, Box<Expression>>,
        environment_opt: Option<Environment>,
    ) -> Result<Vec<(Option<Pattern>, Box<Expression>)>, InterpreterError> {

        let receivers_vec: Vec<(Option<Pattern>, Box<Expression>)> = receivers
            .iter()
            .map(|&(reference_sig, body)| (reference_sig, body))
            .collect();


        receivers_vec
            .iter()
            .sort_by(|a, b| {
                b.0.get_precedence().cmp(&a.0.get_precedence())
            })
            .collect()
    }*/

    fn call_method(
        &mut self,
        call: Call,
        // An optional environment used for variables in local scope.
        environment_opt: Option<Environment>,
    ) -> Result<Box<Expression>, InterpreterError> {
        if self.recursion_level >= 4 {
            return Err(InterpreterError::TooMuchRecursion)
        }

        println!("HANDLE CALL name: {}", call.name);
        match call.name.as_str() {
            "+" | "-" | "*" | "/" => return self.do_arithmetic(call),

            _ => {},
        }

        Ok(Box::new(
            Expression {
                kind: ExpressionKind::Identifier,
                start_pos: 0,
                end_pos: 0,
                lexeme: call.name.clone(),
            }
        ))
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
    UnexpectedType { expected: String, found: Option<String> },
    MethodAlreadyExists,
    SignatureAlreadyExists,
    NoMatchingReceiver,
    NoMatchingMultimethod,
    NoMatchingVariable,
    TooMuchRecursion,
}

#[derive(Debug, Clone)]
pub struct Multimethod {
    pub receivers: Vec<Receiver>,
}

#[derive(Debug, Clone)]
pub struct Receiver {
    pub signature: Option<Pattern>,
    pub body:      Box<Expression>,
}

impl Receiver {
    pub fn from(signature: Option<Pattern>, body: Box<Expression>) -> Self {
        Self {
            signature,
            body,
        }
    }
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

    pub fn call(&self, call: Call, environment_opt: Option<Environment>) -> Result<Box<Expression>, InterpreterError> {
        let mut matching_receivers = self.find_matching_receivers(call.clone(), environment_opt)?;

        // Sort the receivers so the one with the highest precedence value goes first.
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