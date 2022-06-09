mod arithmetic;

pub use self::arithmetic::*;

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use uuid::Uuid;

use magc::type_system::Typed;
use magc::types::*;

use crate::interpreter::InterpreterError;
use crate::types::Multimethod;

use magc::types::{
    Expression,
    ExpressionKind,
    Block,
    Call,
    Conditional,
    Method,
    Infix,
    Prefix,
    Literal,
};

use crate::types::Pattern;

#[derive(Debug, Clone, Eq)]
pub struct Obj {
    pub uuid: Uuid,
    pub kind: ObjKind,
}

impl Obj {
    pub fn new(kind: ObjKind) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            kind,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match &self.kind {
            ObjKind::Boolean(true) => true,
            _                      => false,
        }
    }
}

impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self.kind {
            ObjKind::Int(int)     => write!(f, "{}", int),
            ObjKind::UInt(uint)   => write!(f, "{}", uint),
            ObjKind::Float(float) => write!(f, "{}", float),
            ObjKind::Boolean(boolean) => write!(f, "{:?}", boolean),
            ObjKind::Nothing => write!(f, "nothing"),

            _ => write!(f, "_"),
        };

        Ok(())
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    Float(String),
    /// A boolean value.
    Boolean(bool),
    /// A sequence of characters encoded in UTF-8.
    String(String),
    Nothing,
    /// A list of objects enclosed in brackets.
    List(Option<Box<Obj>>),
    /// A type which represents a Mag expression.
    Expression(Expression),
}
/*
    /// An `if` expression running different branches of code based on a given condition.
    Conditional(Conditional),
    /// A list of expressions enclosed in brackets, like `[1, 2, 3]`.
    ///
    /// The optional single child expression allows putting zero or more entries into
    /// the list, making use of the pair pattern if there is more than one expression.
    List(Option<Box<Expression>>),
    /// A literal value like `23.4` or `"hello"`.
    Literal(Literal),
    /// A value, tuple, field or variable pattern.
    Pattern(Pattern),
    /// A reference to a type, like `Int32`.
    Type,
    /// An expression with a prefix operator.
    Prefix(Prefix),
    /// Two expressions with an infix operator in between.
    Infix(Infix),
    /// An invocation of a method, like `print("Hello, World!")`
    Call(Call),
    /// A definition of a method with a given name, signature and body.
    Method(Method),
    /// A first-class chunk of code that can be passed around as a value.
    Block(Block),
    Identifier,
*/

impl From<Expression> for Obj {
    fn from(expression: Expression) -> Self {
        let kind = match expression.kind {
            ExpressionKind::List(optional_expr) => {
                if let Some(inner_expr) = optional_expr {
                    ObjKind::List(Some(Box::new(Obj::from(*inner_expr))))
                } else {
                    ObjKind::List(None)
                }
            },

            ExpressionKind::Pattern(pattern) => ObjKind::Pattern(Pattern::from(pattern)),
            ExpressionKind::Type             => ObjKind::Type(expression.lexeme),

            ExpressionKind::Literal(literal) => {
                match literal {
                    Literal::Int     => ObjKind::Int(expression.lexeme.parse::<i64>().unwrap()),
                    Literal::Float   => ObjKind::Float(expression.lexeme),
                    Literal::String  => ObjKind::String(expression.lexeme),
                    Literal::Boolean => ObjKind::Boolean(expression.lexeme.parse::<bool>().unwrap()),
                }
            },

            _ => ObjKind::Expression(expression),
        };

        Obj::new(kind)
    }
}

impl Typed for Obj {
    fn get_type(&self) -> Option<String> {
        Some(match self.kind.clone() {
            ObjKind::Multimethod(_) => String::from("Multimethod"),
            ObjKind::Pattern(pattern)     => {
                match pattern {
                    Pattern::Field(_)    => String::from("FieldPattern"),
                    Pattern::Pair(_)     => String::from("PairPattern"),
                    Pattern::Tuple(_)    => String::from("TuplePattern"),
                    Pattern::Value(_)    => String::from("ValuePattern"),
                    Pattern::Variable(_) => String::from("VariablePattern"),
                }
            },
            ObjKind::Int(_)         => String::from("Int"),
            ObjKind::UInt(_)        => String::from("UInt"),
            ObjKind::Float(_)       => String::from("Float"),
            ObjKind::String(_)      => String::from("String"),
            ObjKind::Boolean(_)     => String::from("Boolean"),
            ObjKind::List(_)        => String::from("List"),

            ObjKind::Expression(expression)  => return expression.get_type(),
            ObjKind::Type(type_id)           => type_id,

            ObjKind::Nothing => String::from("Nothing"),
        })
    }
}