pub enum Obj {
    /// An instance of a multimethod which is able to handle method calls.
    Multimethod,
    /// A capitalized type identifier.
    Type,
    /// A first-class chunk of code that can be passed around as a value.
    BlockExpression,
    /// A call of a method with a given set of arguments.
    CallExpression,
    /// A definition of a single multimethod receiver, with a given signature and body.
    MethodExpression,
    /// An expression that contains two expressions with an operator in between.
    InfixExpression,
    /// An expression which has an operator in front.
    PrefixExpression,
}