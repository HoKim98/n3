use std::ops::{Deref, DerefMut};

use crate::ast;
use crate::code::Code;
use crate::context::{Build, CloneSafe, Context};
use crate::error::{Result, TensorNodeError};
use crate::execs::ExecIR;
use crate::externs::ExternIR;
use crate::graph::{RefGraph, Values};
use crate::nodes::{builtins, ASTBuild, NodeIR, NodeRoot};
use crate::seed::Seed;

#[derive(Default, Debug, PartialEq)]
pub struct TensorGraph(Vec<TensorNode>);

impl Deref for TensorGraph {
    type Target = Vec<TensorNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TensorGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum TensorNode {
    Node(NodeIR),
    Extern(ExternIR),
    Exec(ExecIR),
}

#[derive(Clone, Debug)]
pub struct IRData {
    pub id: u64,
    pub name: String,
    pub graph: RefGraph,
    pub input: ast::Outs,
    pub output: ast::Outs,
}

impl PartialEq for IRData {
    fn eq(&self, other: &Self) -> bool {
        // id should not be compared
        self.name.eq(&other.name)
            && self.graph.eq(&other.graph)
            && self.input.eq(&other.input)
            && self.output.eq(&other.output)
    }
}

impl From<NodeIR> for TensorNode {
    fn from(node: NodeIR) -> Self {
        Self::Node(node)
    }
}

impl From<ExternIR> for TensorNode {
    fn from(node: ExternIR) -> Self {
        Self::Extern(node)
    }
}

impl From<ExecIR> for TensorNode {
    fn from(node: ExecIR) -> Self {
        Self::Exec(node)
    }
}

impl From<Vec<TensorNode>> for TensorGraph {
    fn from(nodes: Vec<TensorNode>) -> Self {
        Self(nodes)
    }
}

impl TensorGraph {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new_one(node: TensorNode) -> Self {
        Self(vec![node])
    }

    pub fn is_some(&self) -> bool {
        !self.is_empty()
    }

    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        let input_node = &self.0[0];
        if input_node.is_input() {
            input_node.get_output_shapes()
        } else {
            input_node.get_input_shapes()
        }
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        for node in self.0.iter().rev() {
            if let Some(shapes) = node.get_output_shapes() {
                let shapes_ref = shapes.0.borrow();

                // filter dynamic size
                if shapes_ref.len() == 1 {
                    if let Some(None) = shapes_ref.get("x") {
                        continue;
                    }
                }
                return Some(shapes);
            }
        }
        self.0.last().map(|x| x.get_output_shapes()).flatten()
    }

    pub fn try_borrow_extern_node(&self) -> Option<&ExternIR> {
        if self.0.len() == 1 {
            self.0[0].try_borrow_extern()
        } else {
            None
        }
    }

    pub fn build(self, root: &NodeRoot) -> Result<Vec<Code>> {
        self.0.into_iter().map(|x| x.build_to_code(root)).collect()
    }
}

impl TensorNode {
    pub fn is_input(&self) -> bool {
        builtins::INPUTS.contains(&self.name())
    }

    pub fn name(&self) -> &str {
        &self.get_data().name
    }

    fn ty(&self) -> ast::FinalNodeType {
        match self {
            Self::Node(_) | Self::Extern(_) => ast::FinalNodeType::Default,
            Self::Exec(_) => ast::FinalNodeType::Exec,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.get_data().id
    }

    pub fn set_id(&mut self, id: u64) {
        self.get_data_mut().id = id;
    }

    pub fn set_repeat(&mut self, value: Option<ast::Value>) {
        match self {
            Self::Node(node) => node.repeat = value,
            _ => unreachable!("Only the default nodes can repeat."),
        }
    }

    fn get_data(&self) -> &IRData {
        match self {
            Self::Node(node) => &node.data,
            Self::Extern(node) => &node.data,
            Self::Exec(node) => &node.data,
        }
    }

    fn get_data_mut(&mut self) -> &mut IRData {
        match self {
            Self::Node(node) => &mut node.data,
            Self::Extern(node) => &mut node.data,
            Self::Exec(node) => &mut node.data,
        }
    }

    pub fn get_graph(&self) -> &RefGraph {
        &self.get_data().graph
    }

    pub fn get_inputs(&self) -> &ast::Outs {
        &self.get_data().input
    }

    pub fn get_inputs_mut(&mut self) -> &mut ast::Outs {
        &mut self.get_data_mut().input
    }

    pub fn get_outputs_mut(&mut self) -> &mut ast::Outs {
        &mut self.get_data_mut().output
    }

    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        match self {
            Self::Node(node) => node.get_input_shapes(),
            Self::Extern(node) => node.get_input_shapes(),
            Self::Exec(_) => exec_node_cannot_have_shapes(),
        }
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        match self {
            Self::Node(node) => node.get_output_shapes(),
            Self::Extern(node) => node.get_output_shapes(),
            Self::Exec(_) => exec_node_cannot_have_shapes(),
        }
    }

    pub fn build_to_code(self, root: &NodeRoot) -> Result<Code> {
        match self {
            Self::Node(node) => node.build(root),
            Self::Extern(node) => Ok(node.build()?.into()),
            Self::Exec(_) => unreachable!("The ExecIR should be built using ExecIR::build."),
        }
    }

    pub fn apply_variables(&mut self, variables: Values, shortcut: bool) -> Result<()> {
        self.get_graph().borrow().apply(variables, shortcut)
    }

    pub fn unwrap_node(self) -> Result<NodeIR> {
        match self {
            Self::Node(node) => Ok(node),
            _ => TensorNodeError::MismatchedType {
                expected: ast::FinalNodeType::Default,
                given: self.ty(),
            }
            .into(),
        }
    }

    pub fn unwrap_extern(self) -> Result<ExternIR> {
        match self {
            Self::Extern(node) => Ok(node),
            _ => TensorNodeError::MismatchedType {
                expected: ast::FinalNodeType::Default,
                given: self.ty(),
            }
            .into(),
        }
    }

    pub fn unwrap_exec(self) -> Result<ExecIR> {
        match self {
            Self::Exec(node) => Ok(node),
            _ => TensorNodeError::MismatchedType {
                expected: ast::FinalNodeType::Exec,
                given: self.ty(),
            }
            .into(),
        }
    }

    pub fn try_borrow_extern(&self) -> Option<&ExternIR> {
        match self {
            Self::Extern(node) => Some(node),
            _ => None,
        }
    }
}

fn exec_node_cannot_have_shapes() -> ! {
    unreachable!("The exec node cannot have the shapes");
}

impl CloneSafe for TensorGraph {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self(self.0.clone_safe(seed, variables))
    }
}

impl CloneSafe for TensorNode {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        match self {
            Self::Node(node) => node.clone_safe(seed, variables).into(),
            Self::Extern(node) => node.clone_safe(seed, variables).into(),
            Self::Exec(node) => node.clone_safe(seed, variables).into(),
        }
    }
}

impl CloneSafe for IRData {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            graph: self.graph.clone_safe(seed, variables),
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}

impl Build for TensorNode {
    type Output = Self;

    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self::Output> {
        let file = root.parser.parse_file(&source)?;

        // test name
        if file.node.name != name {
            TensorNodeError::MismatchedName {
                expected: name.to_string(),
                given: file.node.name,
            }
            .into()
        } else {
            let mut ctx = Context::new(root);
            file.build(&mut ctx, Default::default())
        }
    }
}

impl IRData {
    pub fn with_tensor_graph(name: String, graph: RefGraph, tensor_graph: &TensorGraph) -> Self {
        Self::with_shapes(
            name,
            graph,
            tensor_graph.get_input_shapes(),
            tensor_graph.get_output_shapes(),
        )
    }

    pub fn with_shapes(
        name: String,
        graph: RefGraph,
        input: Option<&ast::Shapes>,
        output: Option<&ast::Shapes>,
    ) -> Self {
        Self {
            id: 0,
            name,
            graph,
            input: shapes_to_outs(1, input),
            output: shapes_to_outs(1, output),
        }
    }

    pub fn with_no_shapes(name: String, graph: RefGraph) -> Self {
        Self {
            id: 0,
            name,
            graph,
            input: Default::default(),
            output: Default::default(),
        }
    }
}

fn shapes_to_outs(id: u64, shapes: Option<&ast::Shapes>) -> ast::Outs {
    match shapes {
        Some(shapes) => shapes.to_outs(id),
        None => [("x")]
            .iter()
            .map(|x| x.to_string())
            .map(|x| (x.clone(), ast::Out::new(id, x)))
            .collect(),
    }
}
