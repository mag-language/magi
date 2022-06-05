use magc::types::{
    Expression,
    ExpressionKind,
    Conditional,
    Literal,
    Pattern,
    VariablePattern,
    Prefix,
    Infix,
    Call,
    Method,
    Block,
};

use crate::types::{
    Obj,
    ObjKind,
};

/// Convert an [`Expression`] into an [`Obj`].
pub fn expr_to_obj(expression: Expression) -> Obj {
    let kind = match expression.kind {
        ExpressionKind::Conditional(conditional) => ObjKind::ConditionalExpression(conditional),
        ExpressionKind::List(optional_expr)      => ObjKind::List(optional_expr),
        ExpressionKind::Literal(literal)         => {
            match literal {
                Literal::Int     => ObjKind::Int(expression.lexeme.parse::<i64>().unwrap()),
                Literal::Float   => ObjKind::Float(expression.lexeme.parse::<f64>().unwrap()),
                Literal::String  => ObjKind::String(expression.lexeme),
                Literal::Boolean => {
                    match expression.lexeme.as_str() {
                        "true"  => ObjKind::Bool(true),
                        "false" => ObjKind::Bool(false),

                        _ => unreachable!(),
                    }
                },
            }
        },
        ExpressionKind::Pattern(pattern) => ObjKind::Pattern(pattern),
        ExpressionKind::Prefix(prefix)   => ObjKind::PrefixExpression(prefix),
        ExpressionKind::Infix(infix)     => ObjKind::InfixExpression(infix),
        ExpressionKind::Call(call)       => ObjKind::CallExpression(call),
        ExpressionKind::Method(method)   => ObjKind::MethodExpression(method),
        ExpressionKind::Block(block)     => ObjKind::BlockExpression(block),
        ExpressionKind::Identifier       => ObjKind::Pattern(Pattern::Variable(VariablePattern {
            name: Some(expression.lexeme),
            type_id: None,
        })),
        ExpressionKind::Type             => ObjKind::Type(expression.lexeme),
    };

    Obj::new(kind)
}