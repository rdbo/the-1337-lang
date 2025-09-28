#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub declared_type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub identifier: Option<String>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<Type>,
    pub code: CodeBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Common(String),
    Pointer(Box<Type>),
    Function {
        params: Vec<FunctionParam>,
        return_type: Option<Box<Type>>,
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
    FunctionDefinition {
        identifier: String,
        params: Vec<FunctionParam>,
        return_type: Option<Type>,
        code: CodeBlock,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub nodes: Vec<NodeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    AnonymousFunctionDefinition {
        params: Vec<FunctionParam>,
        return_type: Option<Type>,
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
