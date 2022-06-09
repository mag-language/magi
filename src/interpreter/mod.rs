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
    ConditionalVisitor,
    MethodVisitor,
    ValueVisitor,
    InfixVisitor,
    PatternVisitor,
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
    pub visitors: HashMap<String, &'static dyn Visitor>,
    pub recursion_level: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut visitors = HashMap::new();

        visitors.insert("CallExpression".to_string(),        &CallVisitor        as &dyn Visitor);
        visitors.insert("ConditionalExpression".to_string(), &ConditionalVisitor as &dyn Visitor);
        visitors.insert("MethodExpression".to_string(),      &MethodVisitor      as &dyn Visitor);
        visitors.insert("InfixExpression".to_string(),       &InfixVisitor       as &dyn Visitor);

        visitors.insert("Int".to_string(),      &ValueVisitor as &dyn Visitor);
        visitors.insert("UInt".to_string(),     &ValueVisitor as &dyn Visitor);
        visitors.insert("Float".to_string(),    &ValueVisitor as &dyn Visitor);
        visitors.insert("String".to_string(),   &ValueVisitor as &dyn Visitor);
        visitors.insert("Boolean".to_string(), &ValueVisitor as &dyn Visitor);

        visitors.insert("FieldPattern".to_string(),    &PatternVisitor as &dyn Visitor);
        visitors.insert("PairPattern".to_string(),     &PatternVisitor as &dyn Visitor);
        visitors.insert("TuplePattern".to_string(),    &PatternVisitor as &dyn Visitor);
        visitors.insert("ValuePattern".to_string(),    &PatternVisitor as &dyn Visitor);
        visitors.insert("VariablePattern".to_string(), &PatternVisitor as &dyn Visitor);

        Self {
            environment: Environment::new(),
            visitors,
            recursion_level: 0,
        }
    }

    pub fn get_variable(&self, variable_pattern: VariablePattern, optional_env: Option<Environment>) -> InterpreterResult {
        if let Some(env) = optional_env {
            if let Some(value) = env.entries.get(&variable_pattern) {
                Ok(value.clone())
            } else {
                Err(InterpreterError::NoMatchingVariable { variable_pattern })
            }
        } else {
            if let Some(value) = self.environment.entries.get(&variable_pattern) {
                Ok(value.clone())
            } else {
                Err(InterpreterError::NoMatchingVariable { variable_pattern })
            }
        }
    }

    pub fn define_variable(&mut self, variable_pattern: VariablePattern, obj: Obj) -> InterpreterResult {
        if !self.environment.entries.contains_key(&variable_pattern) {
            self.environment.entries.insert(variable_pattern.clone(), Box::new(obj));

            Ok(Box::new(Obj::new(
                ObjKind::Pattern(Pattern::Variable(variable_pattern))
            )))
        } else {
            Err(InterpreterError::VariableAlreadyExists)
        }
    }

    pub fn mutate_variable(&mut self, variable_pattern: VariablePattern, obj: Obj) -> InterpreterResult {
        if self.environment.entries.contains_key(&variable_pattern) {
            // Delete the previous entry and recreate it with the new object.
            self.environment.entries.remove(&variable_pattern);
            self.environment.entries.insert(variable_pattern.clone(), Box::new(obj));

            Ok(Box::new(Obj::new(
                ObjKind::Pattern(Pattern::Variable(variable_pattern))
            )))
        } else {
            Err(InterpreterError::NoMatchingVariable { variable_pattern })
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

            _ => Err(InterpreterError::NoMatchingVisitor),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum InterpreterError {
    Unimplemented,
    UnexpectedType { expected: String, found: Option<String> },
    MethodAlreadyExists,
    SignatureAlreadyExists,
    VariableAlreadyExists,
    NoMatchingReceiver,
    NoMatchingMultimethod,
    NoMatchingVariable { variable_pattern: VariablePattern },
    NoMatchingVisitor,
    /// Raised when the linearization of two patterns fails.
    NoMatch,
    TooMuchRecursion,
}