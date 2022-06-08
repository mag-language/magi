use std::ops::{Add, Sub, Mul, Div};

use super::*;

use crate::interpreter::{
    InterpreterResult,
    InterpreterError,
};

impl Add for Obj {
    type Output = Result<Self, InterpreterError>;

    fn add(self, other: Self) -> Result<Self, InterpreterError> {
        self::add(self, other)
    }
}

impl Sub for Obj {
    type Output = Result<Self, InterpreterError>;

    fn sub(self, other: Self) -> Result<Self, InterpreterError> {
        self::sub(self, other)
    }
}

impl Mul for Obj {
    type Output = Result<Self, InterpreterError>;

    fn mul(self, other: Self) -> Result<Self, InterpreterError> {
        self::mul(self, other)
    }
}

impl Div for Obj {
    type Output = Result<Self, InterpreterError>;

    fn div(self, other: Self) -> Result<Self, InterpreterError> {
        self::div(self, other)
    }
}

fn add(o1: Obj, o2: Obj) -> Result<Obj, InterpreterError> {
    let kind = match (o1.kind.clone(), o2.kind.clone()) {
        (ObjKind::Int(n1), ObjKind::Int(n2))     => ObjKind::Int(n1 + n2),
        (ObjKind::Int(n1), ObjKind::UInt(n2))    => ObjKind::Int(n1 + n2 as i64),
        (ObjKind::Int(n1), ObjKind::Float(n2))   => ObjKind::Float(format!("{}", n1 as f64 + n2.parse::<f64>().unwrap())),

        (ObjKind::UInt(n1), ObjKind::Int(n2))    => ObjKind::UInt(n1 + n2 as u64),
        (ObjKind::UInt(n1), ObjKind::UInt(n2))   => ObjKind::UInt(n1 + n2),
        (ObjKind::UInt(n1), ObjKind::Float(n2))  => ObjKind::Float(format!("{}", n1 as f64 + n2.parse::<f64>().unwrap())),

        (ObjKind::Float(n1), ObjKind::Int(n2))   => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() + n2 as f64)),
        (ObjKind::Float(n1), ObjKind::UInt(n2))  => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() + n2 as f64)),
        (ObjKind::Float(n1), ObjKind::Float(n2)) => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() + n2.parse::<f64>().unwrap())),

        _ => return Err(InterpreterError::UnexpectedType {
            expected: String::from("Int | UInt | Float"),
            found: Some(format!("({:?}, {:?})", o1, o2)),
        })
    };

    Ok(Obj::new(kind))
}

fn sub(o1: Obj, o2: Obj) -> Result<Obj, InterpreterError> {
    let kind = match (o1.kind.clone(), o2.kind.clone()) {
        (ObjKind::Int(n1), ObjKind::Int(n2))     => ObjKind::Int(n1 - n2),
        (ObjKind::Int(n1), ObjKind::UInt(n2))    => ObjKind::Int(n1 - n2 as i64),
        (ObjKind::Int(n1), ObjKind::Float(n2))   => ObjKind::Float(format!("{}", n1 as f64 - n2.parse::<f64>().unwrap())),

        (ObjKind::UInt(n1), ObjKind::Int(n2))    => ObjKind::UInt(n1 - n2 as u64),
        (ObjKind::UInt(n1), ObjKind::UInt(n2))   => ObjKind::UInt(n1 - n2),
        (ObjKind::UInt(n1), ObjKind::Float(n2))  => ObjKind::Float(format!("{}", n1 as f64 - n2.parse::<f64>().unwrap())),

        (ObjKind::Float(n1), ObjKind::Int(n2))   => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() - n2 as f64)),
        (ObjKind::Float(n1), ObjKind::UInt(n2))  => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() - n2 as f64)),
        (ObjKind::Float(n1), ObjKind::Float(n2)) => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() - n2.parse::<f64>().unwrap())),

        _ => return Err(InterpreterError::UnexpectedType {
            expected: String::from("Int | UInt | Float"),
            found: Some(format!("({:?}, {:?})", o1, o2)),
        })
    };

    Ok(Obj::new(kind))
}

fn mul(o1: Obj, o2: Obj) -> Result<Obj, InterpreterError> {
    let kind = match (o1.kind.clone(), o2.kind.clone()) {
        (ObjKind::Int(n1), ObjKind::Int(n2))     => ObjKind::Int(n1 * n2),
        (ObjKind::Int(n1), ObjKind::UInt(n2))    => ObjKind::Int(n1 * n2 as i64),
        (ObjKind::Int(n1), ObjKind::Float(n2))   => ObjKind::Float(format!("{}", n1 as f64 * n2.parse::<f64>().unwrap())),

        (ObjKind::UInt(n1), ObjKind::Int(n2))    => ObjKind::UInt(n1 * n2 as u64),
        (ObjKind::UInt(n1), ObjKind::UInt(n2))   => ObjKind::UInt(n1 * n2),
        (ObjKind::UInt(n1), ObjKind::Float(n2))  => ObjKind::Float(format!("{}", n1 as f64 * n2.parse::<f64>().unwrap())),

        (ObjKind::Float(n1), ObjKind::Int(n2))   => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() * n2 as f64)),
        (ObjKind::Float(n1), ObjKind::UInt(n2))  => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() * n2 as f64)),
        (ObjKind::Float(n1), ObjKind::Float(n2)) => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() * n2.parse::<f64>().unwrap())),

        _ => return Err(InterpreterError::UnexpectedType {
            expected: String::from("Int | UInt | Float"),
            found: Some(format!("({:?}, {:?})", o1, o2)),
        })
    };

    Ok(Obj::new(kind))
}

fn div(o1: Obj, o2: Obj) -> Result<Obj, InterpreterError> {
    let kind = match (o1.kind.clone(), o2.kind.clone()) {
        (ObjKind::Int(n1), ObjKind::Int(n2))     => ObjKind::Int(n1 / n2),
        (ObjKind::Int(n1), ObjKind::UInt(n2))    => ObjKind::Int(n1 / n2 as i64),
        (ObjKind::Int(n1), ObjKind::Float(n2))   => ObjKind::Float(format!("{}", n1 as f64 / n2.parse::<f64>().unwrap())),

        (ObjKind::UInt(n1), ObjKind::Int(n2))    => ObjKind::UInt(n1 / n2 as u64),
        (ObjKind::UInt(n1), ObjKind::UInt(n2))   => ObjKind::UInt(n1 / n2),
        (ObjKind::UInt(n1), ObjKind::Float(n2))  => ObjKind::Float(format!("{}", n1 as f64 / n2.parse::<f64>().unwrap())),

        (ObjKind::Float(n1), ObjKind::Int(n2))   => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() / n2 as f64)),
        (ObjKind::Float(n1), ObjKind::UInt(n2))  => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() / n2 as f64)),
        (ObjKind::Float(n1), ObjKind::Float(n2)) => ObjKind::Float(format!("{}", n1.parse::<f64>().unwrap() / n2.parse::<f64>().unwrap())),

        _ => return Err(InterpreterError::UnexpectedType {
            expected: String::from("Int | UInt | Float"),
            found: Some(format!("({:?}, {:?})", o1, o2)),
        })
    };

    Ok(Obj::new(kind))
}

