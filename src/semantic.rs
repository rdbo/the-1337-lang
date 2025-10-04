use std::collections::HashMap;

use crate::{CodeBlock, Expression, FunctionParam, Node, NodeInfo, Statement, Type};

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
pub struct SemanticFunctionDefinition {
    pub name: String,
    pub params: Vec<SemanticFunctionParam>,
    pub return_type: Option<SemanticType>,
    pub code: SemanticCodeBlock,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub defined_functions: Vec<SemanticFunctionDefinition>,
    pub scopes: Vec<HashMap<String, Symbol>>,
}

#[derive(Debug, Clone)]
pub struct SemanticExpression {
    pub eval_type: Type,
    pub expr: Expression,
}

#[derive(Debug, Clone)]
pub enum SemanticNode {
    Statement(Statement),
    Expression(SemanticExpression),
    SemanticCodeBlock(SemanticCodeBlock),
}

#[derive(Debug, Clone)]
pub struct SemanticCodeBlock {
    pub locals: HashMap<String, Symbol>,
    pub nodes: Vec<SemanticNode>,
}

pub struct SemanticAnalyzer<'a> {
    program: Program,
    nodes: &'a Vec<NodeInfo>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(nodes: &'a Vec<NodeInfo>) -> Self {
        let program = Program {
            scopes: Vec::from([HashMap::new()]),
            defined_functions: vec![],
        };

        Self { program, nodes }
    }

    fn analyze_extern(&mut self, identifier: String, declared_type: Type) -> Result<(), String> {
        let sem_type = SemanticType::try_from(declared_type)?;
        self.program
            .scopes
            .last_mut()
            .ok_or("no scope available".to_string())?
            .insert(
                identifier.clone(),
                Symbol {
                    name: identifier,
                    sem_type,
                },
            );

        Ok(())
    }

    fn analyze_function_definition(
        &mut self,
        identifier: String,
        params: Vec<FunctionParam>,
        return_type: Option<Type>,
        code: CodeBlock,
    ) -> Result<SemanticFunctionDefinition, String> {
        let sem_ret_type = if let Some(ret_type) = return_type {
            SemanticType::try_from(ret_type).ok()
        } else {
            None
        };

        let sem_params = params
            .into_iter()
            .map(|p| {
                SemanticType::try_from(p.declared_type).map(|t| SemanticFunctionParam {
                    name: p.name,
                    declared_type: t,
                })
            })
            .collect::<Result<Vec<SemanticFunctionParam>, _>>()?;

        // Push new scope for the function,
        // initially containing its parameters
        let scope = sem_params
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    Symbol {
                        name: p.name.clone(),
                        sem_type: p.declared_type.clone(),
                    },
                )
            })
            .collect::<HashMap<String, Symbol>>();

        // let sem_code = self.analyze_codeblock(code, scope)?; // TODO
        let sem_code = SemanticCodeBlock {
            locals: scope,
            nodes: vec![],
        };

        Ok(SemanticFunctionDefinition {
            name: identifier,
            params: sem_params,
            return_type: sem_ret_type,
            code: sem_code,
        })
    }

    fn analyze_node(&mut self, node: &NodeInfo) -> Result<(), String> {
        match node.node.clone() {
            Node::Statement(Statement::Extern {
                identifier,
                declared_type,
            }) => {
                self.analyze_extern(identifier, declared_type)?;
            }
            Node::Statement(Statement::FunctionDefinition {
                identifier,
                params,
                return_type,
                code,
            }) => {
                self.analyze_function_definition(identifier, params, return_type, code)?;
            }

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
