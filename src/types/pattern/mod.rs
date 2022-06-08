//! A runtime version of the pattern type from `magc` which adds support for pattern matching.

mod field;
mod pair;
mod tuple;
mod value;
mod variable;

use std::collections::HashMap;

use magc::types::Pattern as MagcPattern;

pub use self::field::FieldPattern;
pub use self::pair::PairPattern;
pub use self::tuple::TuplePattern;
pub use self::value::ValuePattern;
pub use self::variable::VariablePattern;

use crate::types::{
    Obj,
    Environment,
};
use crate::interpreter::InterpreterError;

pub type MatchResult = Result<Environment, InterpreterError>;

#[derive(Debug, Clone, Eq, PartialEq)]
/// A runtime version of the pattern type from `magc` which adds support for pattern matching.
pub enum Pattern {
    Field(FieldPattern),
    Pair(PairPattern),
    Tuple(TuplePattern),
    Value(ValuePattern),
    Variable(VariablePattern),
}

impl From<MagcPattern> for Pattern {
    fn from(p: MagcPattern) -> Self {
        match p {
            MagcPattern::Field(field_pattern) => Self::Field(FieldPattern {
                name: field_pattern.name,
                value: Box::new(Pattern::from(*field_pattern.value)),
            }),

            MagcPattern::Pair(pair_pattern) => Self::Pair(PairPattern {
                left:  Box::new(Pattern::from(*pair_pattern.left)),
                right: Box::new(Pattern::from(*pair_pattern.right)),
            }),

            MagcPattern::Tuple(tuple_pattern) => Self::Tuple(TuplePattern {
                child: Box::new(Pattern::from(*tuple_pattern.child)),
            }),

            MagcPattern::Value(value_pattern) => Self::Value(ValuePattern {
                obj: Box::new(Obj::from(*value_pattern.expression)),
            }),

            MagcPattern::Variable(variable_pattern) => Self::Variable(VariablePattern {
                name:    variable_pattern.name,
                type_id: variable_pattern.type_id,
            }),
        }
    }
}

impl Pattern {
    /// Compare this pattern with another and return any destructured variables.
    ///
    /// This function recursively calls itself and the respective pattern methods
    /// to evaluate whether or not a tree of patterns matches with another. A typeless
    /// variable matches any value pattern, for example.
    pub fn linearize(&self, other: Pattern) -> MatchResult {
        match self {
            Pattern::Field(reference)    => self.linearize_field(reference.clone(), other),
            Pattern::Tuple(reference)    => self.linearize_tuple(reference.clone(), other),
            Pattern::Value(reference)    => self.linearize_value(reference.clone(), other),
            Pattern::Variable(reference) => self.linearize_variable(reference.clone(), other),
            Pattern::Pair(reference)     => self.linearize_pair(reference.clone(), other),
        }
    }

    pub fn matches_with(&self, other: Pattern) -> bool {
        match self.linearize(other) {
            Ok(_)  => true,
            Err(_) => false,
        }
    }

    pub fn get_precedence(&self) -> usize {
        match self {
            Pattern::Value(_) => 2,
            _                 => 1,
        }
    }

    fn linearize_field(&self, reference: FieldPattern, other: Pattern) -> MatchResult {
        if let Pattern::Field(given) = other {
            if given.name != reference.name { return Err(InterpreterError::NoMatch) }

            given.value.linearize(*reference.value)
        } else {
            Err(InterpreterError::NoMatch)
        }
    }

    fn linearize_tuple(&self, reference: TuplePattern, other: Pattern) -> MatchResult {
        if let Pattern::Tuple(TuplePattern { child: other_pattern }) = other {
            reference.child.linearize(*other_pattern)
        } else {
            Err(InterpreterError::NoMatch)
        }
    }

    fn linearize_value(&self, reference: ValuePattern, other: Pattern) -> MatchResult {
        if let Pattern::Value(ValuePattern { obj }) = other {
            if reference.obj.kind == obj.kind {
                Ok(Environment::empty())
            } else {
                Err(InterpreterError::NoMatch)
            }
        } else {
            Err(InterpreterError::NoMatch)
        }
    }

    fn linearize_variable(&self, reference: VariablePattern, other: Pattern) -> MatchResult {
        let mut variables = HashMap::new();

        if let Some(name) = reference.name {
            // Extract value into environment and skip type checking for now.
            if let Pattern::Value(ValuePattern { obj }) = other {
                variables.insert(VariablePattern { name: Some(name), type_id: None }, obj);
            } else {
                // TODO: add proper error handling here!
                return Err(InterpreterError::NoMatch)
            }
        }
        
        Ok(Environment::from(variables))
    }

    fn linearize_pair(&self, reference: PairPattern, other: Pattern) -> MatchResult {
        if let Pattern::Pair(PairPattern { left, right }) = other {
            let mut left_map = reference.left.linearize(*left)?;
            let right_map    = reference.right.linearize(*right)?;

            left_map.extend(right_map);

            Ok(left_map)
        } else {
            Err(InterpreterError::NoMatch)
        }
    }
}