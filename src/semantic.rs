use std::collections::HashMap;

use crate::{CodeBlock, Expression, Node, NodeInfo, Statement, Type};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub sem_type: SemanticType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticFunctionParam {
    pub name: String,
    pub declared_type: SemanticType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticType {
    Int8,
    Int32,
    Pointer(Box<SemanticType>),
    Function {
        params: Vec<SemanticFunctionParam>,
        return_type: Option<Box<SemanticType>>,
    },
}

impl TryFrom<Type> for SemanticType {
    type Error = String;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        let mappings: HashMap<&'static str, SemanticType> =
            HashMap::from([("i8", SemanticType::Int8), ("i32", SemanticType::Int32)]);

        match value {
            Type::Common(t) => mappings
                .get::<str>(t.as_ref())
                .cloned()
                .ok_or_else(|| format!("type '{:?}' does not exist", t))
                .map(|x| x.to_owned()),
            Type::Pointer(p) => {
                let pointee = Self::try_from(*p)?;
                Ok(SemanticType::Pointer(Box::new(pointee)))
            }
            Type::Function {
                params,
                return_type,
            } => {
                let sem_ret_type = if let Some(ret_type) = return_type {
                    Self::try_from(*ret_type).map(|x| Box::new(x)).ok()
                } else {
                    None
                };

                let sem_params = params
                    .into_iter()
                    .map(|p| {
                        Self::try_from(p.declared_type).map(|t| SemanticFunctionParam {
                            name: p.name,
                            declared_type: t,
                        })
                    })
                    .collect::<Result<Vec<SemanticFunctionParam>, _>>()?;

                Ok(SemanticType::Function {
                    params: sem_params,
                    return_type: sem_ret_type,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefinedFunction {
    pub name: String,
    pub params: Vec<SemanticFunctionParam>,
    pub locals: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub global_scope: HashMap<String, Symbol>,
    pub defined_functions: Vec<DefinedFunction>,
}

#[derive(Debug, Clone)]
pub enum SemanticNode {
    Invalid(NodeInfo),
    Statement(Statement),
    Expression { eval_type: Type, expr: Expression },
}

#[derive(Debug, Clone)]
pub struct SemanticNodeInfo {
    pub semantic_node: SemanticNode,
    pub index: usize,
}

pub struct SemanticAnalyzer<'a> {
    program: Program,
    nodes: &'a Vec<NodeInfo>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(nodes: &'a Vec<NodeInfo>) -> Self {
        let program = Program {
            global_scope: HashMap::new(),
            defined_functions: vec![],
        };

        Self { program, nodes }
    }

    fn analyze_node(&mut self, node: &NodeInfo) -> Result<(), String> {
        match &node.node {
            Node::Statement(Statement::Extern {
                identifier,
                declared_type,
            }) => {
                let sem_type = SemanticType::try_from(declared_type.clone())?;
                self.program.global_scope.insert(
                    identifier.to_owned(),
                    Symbol {
                        name: identifier.clone(),
                        sem_type,
                    },
                );
            }
            Node::Statement(Statement::FunctionDefinition {
                identifier,
                params,
                return_type,
                code,
            }) => {}

            n => {
                return Err(format!("unknown node: {:?}", n));
            }
        }

        Ok(())
    }

    pub fn analyze(mut self) -> Result<Program, String> {
        for node in self.nodes {
            self.analyze_node(node)?;
        }

        Ok(self.program)
    }
}
