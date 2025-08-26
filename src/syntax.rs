// TODO: Don't require a Type for void

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Common(String),
    Pointer(Box<Type>),
    Function {
        params: Vec<(String, Type)>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Extern {
        identifier: String,
        declared_type: Type,
    },
    Return {
        value: Expression,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    FunctionDefinition {
        params: Vec<(String, Type)>,
        return_type: Option<Type>,
        expressions: Vec<Expression>,
    },
    Declare {
        identifier: String,
        declared_type: Type,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Invalid,
    Statement(Statement),
    Expression(Expression),
}
