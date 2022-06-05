use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use uuid::Uuid;

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

#[derive(Debug, Clone)]
pub struct Obj {
    uuid: Uuid,
    kind: ObjKind,
}

impl Obj {
    pub fn new(kind: ObjKind) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            kind,
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjKind {
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

enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

fn calculate(method: Arithmetic, this: Obj, other: Obj) -> Result<Obj, InterpreterError> {
    let kind = match this.kind {
        ObjKind::Int(this) => {
            Ok(match other.kind {
                ObjKind::Int(other_int)     => {
                    match method {
                        Arithmetic::Add => ObjKind::Int(this + other_int),
                        Arithmetic::Sub => ObjKind::Int(this - other_int),
                        Arithmetic::Mul => ObjKind::Int(this * other_int),
                        Arithmetic::Div => ObjKind::Int(this / other_int),
                        Arithmetic::Mod => ObjKind::Int(this % other_int),
                    }
                },
                ObjKind::UInt(other_uint)   => {
                    match method {
                        Arithmetic::Add => ObjKind::Int(this + other_uint as i64),
                        Arithmetic::Sub => ObjKind::Int(this - other_uint as i64),
                        Arithmetic::Mul => ObjKind::Int(this * other_uint as i64),
                        Arithmetic::Div => ObjKind::Int(this / other_uint as i64),
                        Arithmetic::Mod => ObjKind::Int(this % other_uint as i64),
                    }
                },
                // TODO: report lossy conversion
                ObjKind::Float(other_float) => {
                    match method {
                        Arithmetic::Add => ObjKind::Int(this + other_float as i64),
                        Arithmetic::Sub => ObjKind::Int(this - other_float as i64),
                        Arithmetic::Mul => ObjKind::Int(this * other_float as i64),
                        Arithmetic::Div => ObjKind::Int(this / other_float as i64),
                        Arithmetic::Mod => ObjKind::Int(this % other_float as i64),
                    }
                },

                _ => return Err(InterpreterError::UnexpectedType {
                    expected: String::from("Int | UInt | Float"),
                    found: None,
                }),
            })
        },

        ObjKind::Float(this) => {
            Ok(match other.kind {
                ObjKind::Int(other_int)     => {
                    match method {
                        Arithmetic::Add => ObjKind::Float(this + other_int as f64),
                        Arithmetic::Sub => ObjKind::Float(this - other_int as f64),
                        Arithmetic::Mul => ObjKind::Float(this * other_int as f64),
                        Arithmetic::Div => ObjKind::Float(this / other_int as f64),
                        Arithmetic::Mod => ObjKind::Float(this % other_int as f64),
                    }
                },
                ObjKind::UInt(other_uint)   => {
                    match method {
                        Arithmetic::Add => ObjKind::Float(this + other_uint as f64),
                        Arithmetic::Sub => ObjKind::Float(this - other_uint as f64),
                        Arithmetic::Mul => ObjKind::Float(this * other_uint as f64),
                        Arithmetic::Div => ObjKind::Float(this / other_uint as f64),
                        Arithmetic::Mod => ObjKind::Float(this % other_uint as f64),
                    }
                },
                // TODO: report lossy conversion
                ObjKind::Float(other_float) => {
                    match method {
                        Arithmetic::Add => ObjKind::Float(this + other_float),
                        Arithmetic::Sub => ObjKind::Float(this - other_float),
                        Arithmetic::Mul => ObjKind::Float(this * other_float),
                        Arithmetic::Div => ObjKind::Float(this / other_float),
                        Arithmetic::Mod => ObjKind::Float(this % other_float),
                    }
                },

                _ => return Err(InterpreterError::UnexpectedType {
                    expected: String::from("Int | UInt | Float"),
                    found: None,
                }),
            })
        },

        ObjKind::UInt(this) => {
            Ok(match other.kind {
                ObjKind::Int(other_int)     => {
                    match method {
                        Arithmetic::Add => ObjKind::UInt(this + other_int as u64),
                        Arithmetic::Sub => ObjKind::UInt(this - other_int as u64),
                        Arithmetic::Mul => ObjKind::UInt(this * other_int as u64),
                        Arithmetic::Div => ObjKind::UInt(this / other_int as u64),
                        Arithmetic::Mod => ObjKind::UInt(this % other_int as u64),
                    }
                },
                ObjKind::UInt(other_uint)   => {
                    match method {
                        Arithmetic::Add => ObjKind::UInt(this + other_uint),
                        Arithmetic::Sub => ObjKind::UInt(this - other_uint),
                        Arithmetic::Mul => ObjKind::UInt(this * other_uint),
                        Arithmetic::Div => ObjKind::UInt(this / other_uint),
                        Arithmetic::Mod => ObjKind::UInt(this % other_uint),
                    }
                },
                ObjKind::Float(other_float) => {
                    match method {
                        Arithmetic::Add => ObjKind::Float(this as f64 + other_float),
                        Arithmetic::Sub => ObjKind::Float(this as f64 - other_float),
                        Arithmetic::Mul => ObjKind::Float(this as f64 * other_float),
                        Arithmetic::Div => ObjKind::Float(this as f64 / other_float),
                        Arithmetic::Mod => ObjKind::Float(this as f64 % other_float),
                    }
                },

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
    };

    Ok(Obj::new(kind?))
}

impl Add for Obj {
    type Output = Result<Self, InterpreterError>;

    fn add(self, other: Self) -> Result<Self, InterpreterError> {
        self::calculate(Arithmetic::Add, self, other)
    }
}

impl Sub for Obj {
    type Output = Result<Self, InterpreterError>;

    fn sub(self, other: Self) -> Result<Self, InterpreterError> {
        self::calculate(Arithmetic::Sub, self, other)
    }
}

impl Mul for Obj {
    type Output = Result<Self, InterpreterError>;

    fn mul(self, other: Self) -> Result<Self, InterpreterError> {
        self::calculate(Arithmetic::Mul, self, other)
    }
}

impl Div for Obj {
    type Output = Result<Self, InterpreterError>;

    fn div(self, other: Self) -> Result<Self, InterpreterError> {
        self::calculate(Arithmetic::Div, self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ObjKind::*;

    #[test]
    fn add_int_to_int() {
        assert_eq!(
            (Obj::new(Int(32)) + (Obj::new(Int(32)))).unwrap(),
            Obj::new(Int(64)),
        )
    }
}