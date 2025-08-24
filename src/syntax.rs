pub enum Type {
    Common(String),
    Pointer(Box<Type>),
    Function {
        params: Vec<Type>,
        return_type: Option<Box<Type>>,
    },
}

pub enum Statement {
    Extern {
        identifier: String,
        declared_type: Type,
    },
    Return {
        value: Expression,
    },
}

pub enum Expression {
    Identifier(String),
    Function {
        params: Vec<Type>,
        return_type: Option<Type>,
        expressions: Vec<Expression>,
    },
    DeclareAndAssign {
        identifier: String,
        value: Box<Expression>,
    },
    FunctionCall {
        identifier: String,
        params: Vec<Expression>,
    },
}

pub enum SyntaxNode {
    Statement(Statement),
    Expression(Expression),
}
