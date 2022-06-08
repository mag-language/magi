pub mod visitors;

use std::collections::{HashMap, BTreeMap};

use crate::types::{
    Environment,
    Multimethod,
    Obj,
    ObjKind,
    VariablePattern,
};

use self::visitors::{
    Visitor,
    CallVisitor,
    MethodVisitor,
    ValueVisitor,
};

use magc::type_system::Typed;

use magc::types::{
    Literal,
    Expression,
    ExpressionKind,
    Method,
    Call,
    Infix,
    Block,
};

use crate::types::pattern::Pattern;

pub type InterpreterResult = Result<Box<Obj>, InterpreterError>;

pub struct Interpreter {
    pub environment: Environment,
    pub methods: HashMap<String, Multimethod>,
    pub visitors: HashMap<String, &'static dyn Visitor>,
    pub recursion_level: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut visitors = HashMap::new();

        visitors.insert("CallExpression".to_string(),   &CallVisitor   as &dyn Visitor);
        visitors.insert("MethodExpression".to_string(), &MethodVisitor as &dyn Visitor);

        visitors.insert("Int".to_string(),    &ValueVisitor as &dyn Visitor);
        visitors.insert("UInt".to_string(),   &ValueVisitor as &dyn Visitor);
        visitors.insert("Float".to_string(),  &ValueVisitor as &dyn Visitor);
        visitors.insert("String".to_string(), &ValueVisitor as &dyn Visitor);

        Self {
            environment: Environment::new(),
            methods:     HashMap::new(),
            visitors,
            recursion_level: 0,
        }
    }

    pub fn get_variable(&self, pattern: VariablePattern) -> InterpreterResult {
        if let Some(value) = self.environment.entries.get(&pattern) {
            Ok(value.clone())
        } else {
            Err(InterpreterError::NoMatchingVariable)
        }
    }

    pub fn define_global(&self, variable_pattern: VariablePattern, obj: Obj) -> InterpreterResult {
        if let None = self.environment.entries.get(&variable_pattern) {
            Ok(Box::new(obj))
        } else {
            Err(InterpreterError::NoMatchingVariable)
        }
    }

    pub fn evaluate_expr(
        &mut self,
        // The expression to evaluate.
        expression: Box<Expression>,
        // An optional environment used for variables in local scope.
        optional_env: Option<Environment>,
    ) -> Result<Box<Obj>, InterpreterError> {

        self.evaluate(
            Box::new(Obj::from(*expression)),
            optional_env
        )
    }

    /// Interpret a given piece of code and return the result.
    pub fn evaluate(
        &mut self,
        // The expression to evaluate.
        obj: Box<Obj>,
        // An optional environment used for variables in local scope.
        optional_env: Option<Environment>,
    ) -> Result<Box<Obj>, InterpreterError> {

        match self.visitors.get(&obj.get_type().unwrap()) {
            Some(visitor) => visitor.evaluate(self, optional_env, *obj),

            _ => Err(InterpreterError::Unimplemented),
        }
    }

    
/*
    fn evaluate_pattern(
        &mut self,
        pattern: Option<Pattern>,
        optional_env: Option<Environment>,
    ) -> Result<Option<Pattern>, InterpreterError> {
        match pattern {
            None => Ok(None),

            Some(Pattern::Value(value_pattern)) => Ok(
                Some(Pattern::Value(ValuePattern {
                    expression: self.evaluate(value_pattern.expression.clone(), optional_env)?,
                }))
            ),

            Some(Pattern::Variable(variable_pattern)) => {
                if let Some(env) = optional_env {
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
*/
    pub fn get_multimethod(&self, name: &String) -> Result<Multimethod, InterpreterError> {
        if let Some(multimethod) = self.methods.get(name) {
            Ok(multimethod.clone())
        } else {
            Err(InterpreterError::NoMatchingMultimethod)
        }
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
    /// Raised when the linearization of two patterns fails.
    NoMatch,
    TooMuchRecursion,
}