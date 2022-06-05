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

use crate::types::Obj;

/// Convert an [`Expression`] into an [`Obj`].
pub fn expr_to_obj(expression: Expression) -> Obj {
    match expression.kind {
        ExpressionKind::Conditional(conditional) => Obj::ConditionalExpression(conditional),
        ExpressionKind::List(optional_expr)      => Obj::List(optional_expr),
        ExpressionKind::Literal(literal)         => {
            match literal {
                Literal::Int     => Obj::Int(expression.lexeme.parse::<i64>().unwrap()),
                Literal::Float   => Obj::Float(expression.lexeme.parse::<f64>().unwrap()),
                Literal::String  => Obj::String(expression.lexeme),
                Literal::Boolean => {
                    match expression.lexeme.as_str() {
                        "true"  => Obj::Bool(true),
                        "false" => Obj::Bool(false),

                        _ => unreachable!(),
                    }
                },
            }
        },
        ExpressionKind::Pattern(pattern) => Obj::Pattern(pattern),
        ExpressionKind::Prefix(prefix)   => Obj::PrefixExpression(prefix),
        ExpressionKind::Infix(infix)     => Obj::InfixExpression(infix),
        ExpressionKind::Call(call)       => Obj::CallExpression(call),
        ExpressionKind::Method(method)   => Obj::MethodExpression(method),
        ExpressionKind::Block(block)     => Obj::BlockExpression(block),
        ExpressionKind::Identifier       => Obj::Pattern(Pattern::Variable(VariablePattern {
            name: Some(expression.lexeme),
            type_id: None,
        })),
        ExpressionKind::Type             => Obj::Type(expression.lexeme),
    }
}