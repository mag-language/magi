use std::ops::{Add, Sub, Mul, Div};

use crate::interpreter::InterpreterError;
use crate::types::Multimethod;

use magc::types::{
    Expression,
    Block,
    Call,
    Conditional,
    Method,
    Infix,
    Prefix,
    Pattern,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    /// An instance of a multimethod which is able to handle method calls.
    Multimethod(Multimethod),
    /// A pattern that can be matched against another pattern.
    Pattern(Pattern),
    /// A capitalized type identifier.
    Type(String),
    /// A 64-bit signed integer value.
    Int(i64),
    /// A 64-bit unsigned integer value.
    UInt(u64),
    /// A 64-bit float value.
    Float(f64),
    /// A boolean value.
    Bool(bool),
    /// A sequence of characters encoded in UTF-8.
    String(String),
    /// A list of objects enclosed in brackets.
    List(Option<Box<Expression>>),
    /// A first-class chunk of code that can be passed around as a value.
    BlockExpression(Block),
    /// A call of a method with a given set of arguments.
    CallExpression(Call),
    /// An `if` expression that evaluates different branches of code based on a given condition.
    ConditionalExpression(Conditional),
    /// A definition of a single multimethod receiver, with a given signature and body.
    MethodExpression(Method),
    /// An expression that contains two expressions with an operator in between.
    InfixExpression(Infix),
    /// An expression which has an operator in front.
    PrefixExpression(Prefix),
}

impl Add for Obj {
    type Output = Result<Self, InterpreterError>;

    fn add(self, other: Self) -> Result<Self, InterpreterError> {
        match self {
            Obj::Int(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Int(this + other_int),
                    Obj::UInt(other_uint)   => Obj::Int(this + other_uint as i64),
                    // TODO: report lossy conversion
                    Obj::Float(other_float) => Obj::Float(this as f64 + other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::Float(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Float(this + other_int as f64),
                    Obj::UInt(other_uint)   => Obj::Float(this + other_uint as f64),
                    Obj::Float(other_float) => Obj::Float(this + other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::UInt(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::UInt(this + other_int as u64),
                    Obj::UInt(other_uint)   => Obj::UInt(this + other_uint),
                    Obj::Float(other_float) => Obj::Float(this as f64 + other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            _ => Err(InterpreterError::UnexpectedType {
                expected: String::from("Int | UInt | Float"),
                found: None,
            }),
        }
    }
}

impl Sub for Obj {
    type Output = Result<Self, InterpreterError>;

    fn sub(self, other: Self) -> Result<Self, InterpreterError> {
        match self {
            Obj::Int(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Int(this - other_int),
                    Obj::UInt(other_uint)   => Obj::Int(this - other_uint as i64),
                    // TODO: report lossy conversion
                    Obj::Float(other_float) => Obj::Float(this as f64 - other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::Float(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Float(this - other_int as f64),
                    Obj::UInt(other_uint)   => Obj::Float(this - other_uint as f64),
                    Obj::Float(other_float) => Obj::Float(this - other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::UInt(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::UInt(this - other_int as u64),
                    Obj::UInt(other_uint)   => Obj::UInt(this - other_uint),
                    Obj::Float(other_float) => Obj::Float(this as f64 - other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            _ => Err(InterpreterError::UnexpectedType {
                expected: String::from("Int | UInt | Float"),
                found: None,
            }),
        }
    }
}

impl Mul for Obj {
    type Output = Result<Self, InterpreterError>;

    fn mul(self, other: Self) -> Result<Self, InterpreterError> {
        match self {
            Obj::Int(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Int(this * other_int),
                    Obj::UInt(other_uint)   => Obj::Int(this * other_uint as i64),
                    // TODO: report lossy conversion
                    Obj::Float(other_float) => Obj::Float(this as f64 * other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::Float(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Float(this * other_int as f64),
                    Obj::UInt(other_uint)   => Obj::Float(this * other_uint as f64),
                    Obj::Float(other_float) => Obj::Float(this * other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::UInt(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::UInt(this * other_int as u64),
                    Obj::UInt(other_uint)   => Obj::UInt(this * other_uint),
                    Obj::Float(other_float) => Obj::Float(this as f64 * other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            _ => Err(InterpreterError::UnexpectedType {
                expected: String::from("Int | UInt | Float"),
                found: None,
            }),
        }
    }
}

impl Div for Obj {
    type Output = Result<Self, InterpreterError>;

    fn div(self, other: Self) -> Result<Self, InterpreterError> {
        match self {
            Obj::Int(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Int(this / other_int),
                    Obj::UInt(other_uint)   => Obj::Int(this / other_uint as i64),
                    // TODO: report lossy conversion
                    Obj::Float(other_float) => Obj::Float(this as f64 / other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::Float(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::Float(this / other_int as f64),
                    Obj::UInt(other_uint)   => Obj::Float(this / other_uint as f64),
                    Obj::Float(other_float) => Obj::Float(this / other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            Obj::UInt(this) => {
                Ok(match other {
                    Obj::Int(other_int)     => Obj::UInt(this / other_int as u64),
                    Obj::UInt(other_uint)   => Obj::UInt(this / other_uint),
                    Obj::Float(other_float) => Obj::Float(this as f64 / other_float),

                    _ => return Err(InterpreterError::UnexpectedType {
                        expected: String::from("Int | UInt | Float"),
                        found: None,
                    }),
                })
            },

            _ => Err(InterpreterError::UnexpectedType {
                expected: String::from("Int | UInt | Float"),
                found: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_int_to_int() {
        assert_eq!(
            (Obj::Int(32) + Obj::Int(32)).unwrap(),
            Obj::Int(64),
        );
    }
}