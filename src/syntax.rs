#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub declared_type: Type,
}

// TODO: Don't require a Type for void
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Common(String),
    Pointer(Box<Type>),
    Function {
        params: Vec<FunctionParam>,
        return_type: Box<Type>, // TODO: Make it optional
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
pub struct CodeBlock {
    pub nodes: Vec<NodeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    FunctionDefinition {
        params: Vec<FunctionParam>,
        return_type: Type, // TODO: Make it optional
        code: CodeBlock,
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
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Invalid,
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeInfo {
    pub node: Node,
    pub start_index: usize,
    pub end_index: usize,
    pub message: String,
}
