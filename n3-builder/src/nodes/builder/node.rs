use std::collections::BTreeMap;

use super::super::ir::NodeIR;
use super::graph::GraphNodeEntry;
use crate::ast;
use crate::context::{Context, NodeName};
use crate::error::{ExecBuildError, GraphCallError, GraphNodeError, Result};
use crate::execs::ExecIR;
use crate::externs::ExternIR;
use crate::graph::{Graph, RefGraph};
use crate::tensor::{IRData, TensorGraph, TensorNode};

pub trait ASTBuild<'a> {
    type Args;
    type Output;

    fn build(self, ctx: &mut Context<'a>, args: Self::Args) -> Result<Self::Output>;
}

pub struct NodeEntry<'a, 'b>
where
    'a: 'b,
{
    name: NodeName,
    pub graph: RefGraph,
    pub ctx: &'b mut Context<'a>,

    pub tensor_graph: TensorGraph,
    pub last_tensor_id: u64,
}

impl<'a, 'b> NodeEntry<'a, 'b> {
    fn new(name: NodeName, graph: RefGraph, ctx: &'b mut Context<'a>) -> Self {
        Self {
            name,
            graph,
            ctx,
            tensor_graph: Default::default(),
            last_tensor_id: 0,
        }
    }

    fn hint_variables(&mut self, tensor_graph: &mut BTreeMap<u64, ast::GraphNode>) -> Result<()> {
        let graph = self.graph.borrow();
        for (&id, n) in tensor_graph.iter_mut() {
            if let Some(shapes) = &mut n.shapes {
                for (x, shape) in shapes.0.borrow_mut().iter_mut() {
                    if let Some(shape) = shape {
                        let out = ast::Out::new(id, x.clone());
                        *shape = graph.hint(&out, shape)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn add_use(&mut self, name: String, u: ast::Use) -> Result<()> {
        // Step 1. get the source
        // Step 2. build
        // Step 3. store
        todo!()
    }

    fn add_with(&mut self, name: String, with: ast::With) -> Result<()> {
        // Step 1. get the node
        let mut node = self.get(&name)?;

        // Step 2. apply variables
        let args = {
            let graph = self.graph.borrow();
            with.graph
                .into_iter()
                .map(|(k, v)| {
                    let value = graph.replace_to(Some(v))?;
                    Ok((k, value))
                })
                .collect::<Result<_>>()?
        };
        node.apply_variables(args, false)?;

        // Step 3. store
        self.ctx.add_child(&self.name, node);
        Ok(())
    }

    fn add_child(&mut self, child: ast::Node) -> Result<()> {
        // Step 1. convert to file
        let file = ast::File {
            uses: Default::default(),
            node: child,
        };

        // Step 2. build
        let node = file.build(self.ctx, self.name.clone())?;

        // Step 3. store
        self.ctx.add_child(&self.name, node);
        Ok(())
    }

    fn add_tensor_graph(&mut self, node: ast::GraphNode) -> Result<()> {
        let last_id = self.last_tensor_id;
        if node.id < last_id || node.id - last_id != 1 && !(last_id == 0 && node.id == 0) {
            GraphNodeError::MismatchedId {
                expected: last_id + 1,
                given: node.id,
            }
            .into()
        } else {
            let id = node.id;
            GraphNodeEntry { root: self, node }.build()?;

            // store id
            self.last_tensor_id = id;
            Ok(())
        }
    }

    fn build(mut self) -> NodeIR {
        NodeIR {
            data: IRData::with_tensor_graph(
                self.name.pop().unwrap(),
                self.graph,
                &self.tensor_graph,
            ),
            ty: ast::LetNodeType::Default,
            tensor_graph: self.tensor_graph,
            repeat: None,
        }
    }

    pub fn get(&mut self, name: &str) -> Result<TensorNode> {
        self.ctx.get(&self.name, name)
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        for node in self.tensor_graph.iter().rev() {
            if let Some(outputs) = node.get_output_shapes() {
                return Some(outputs);
            }
        }
        None
    }

    pub fn fetch_shape(&self, out: &mut ast::Out) -> Result<Option<ast::Shape>> {
        for node in self.tensor_graph.iter().rev() {
            // test id
            let node_id = node.get_id();
            if let Some(id) = &out.id {
                if node_id > *id {
                    continue;
                }
                if node_id < *id {
                    break;
                }
            }

            if let Some(shapes) = node.get_output_shapes() {
                if let Some(shape) = shapes.0.borrow().get(&out.name) {
                    out.id = Some(node_id);
                    return Ok(shape.as_ref().cloned());
                }
            }
        }
        GraphNodeError::NoSuchInput { out: out.clone() }.into()
    }
}

impl<'a> ASTBuild<'a> for ast::File {
    type Args = NodeName;
    type Output = TensorNode;

    fn build(self, ctx: &mut Context<'a>, parent: Self::Args) -> Result<Self::Output> {
        if self.node.ty.is_extern() {
            return Ok(ExternFile(self).build(ctx, ())?.into());
        }
        if self.node.ty.is_exec() {
            return Ok(ExecFile(self).build(ctx, ())?.into());
        }

        let mut node = self.node;

        let mut name = parent;
        name.push(node.name.clone());

        // Step 1. make a graph
        let graph: RefGraph =
            Graph::try_with_variables(ctx.root.seed.generate(), node.graph, false)?.into();
        ctx.add_graph(name.clone(), graph.clone());

        let mut entry = NodeEntry::new(name, graph, ctx);

        // Step 2. import remote models
        for (name, u) in self.uses {
            entry.add_use(name, u)?;
        }

        // Step 3. hint variables with tensor graph
        entry.hint_variables(&mut node.tensor_graph)?;

        // Step 4. re-define nodes (with)
        for (name, w) in node.withs {
            entry.add_with(name, w)?;
        }

        // Step 5. build children nodes
        for (_, child) in node.children {
            entry.add_child(child)?;
        }

        // Step 6. make a tensor graph
        for (_, n) in node.tensor_graph {
            entry.add_tensor_graph(n)?;
        }

        // Step 7. store
        Ok(entry.build().into())
    }
}

struct ExternNodeEntry<'a, 'b> {
    inner: NodeEntry<'a, 'b>,
    ty: ast::ExternNodeType,

    input: Option<ast::Shapes>,
    output: Option<ast::Shapes>,
}
impl<'a, 'b> ExternNodeEntry<'a, 'b> {
    fn new(inner: NodeEntry<'a, 'b>, ty: ast::ExternNodeType) -> Self {
        Self {
            inner,
            ty,
            input: None,
            output: None,
        }
    }

    fn hint_variables(&mut self, tensor_graph: &mut BTreeMap<u64, ast::GraphNode>) -> Result<()> {
        self.inner.hint_variables(tensor_graph)
    }

    fn test_tensor_graph(&self, nodes: &BTreeMap<u64, ast::GraphNode>) -> Result<()> {
        ExternTensorGraphCondition {
            nodes,
            names: match self.ty {
                ast::ExternNodeType::Default => &["Input", "Output"],
                ast::ExternNodeType::Data => &["Output"],
                ast::ExternNodeType::Optim => &[],
            },
            ty_inputs: Some(ast::GraphInputsType::UseLast),
            args: Some(&[]),
            is_sized: None,
            repeatable: Some(false),
            is_id_zero: true,
        }
        .test()
    }

    fn add_tensor_graph(&mut self, node: ast::GraphNode) {
        let target = match self.ty {
            ast::ExternNodeType::Default => {
                if node.id == 0 {
                    &mut self.input
                } else {
                    &mut self.output
                }
            }
            ast::ExternNodeType::Data => &mut self.output,
            ast::ExternNodeType::Optim => {
                unreachable!("the optim node cannot have the tensor graph")
            }
        };
        *target = node.shapes;
    }

    fn build(mut self) -> NodeIR {
        let name = self.inner.name.pop().unwrap();

        let extern_node = ExternIR::new(name.clone(), self.inner.graph, self.input, self.output);
        let graph = extern_node.data.graph.clone();

        let tensor_graph = TensorGraph::new_one(extern_node.into());

        NodeIR {
            data: IRData::with_tensor_graph(name, graph, &tensor_graph),
            ty: ast::LetNodeType::Extern(self.ty),
            tensor_graph,
            repeat: None,
        }
    }
}

struct ExternFile(ast::File);
impl<'a> ASTBuild<'a> for ExternFile {
    type Args = ();
    type Output = NodeIR;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        let file = self.0;
        let mut node = file.node;

        let ty = node.ty.unwrap_extern();

        // Step 1. make a graph
        let graph = Graph::try_with_variables(ctx.root.seed.generate(), node.graph, false)?.into();

        let entry = NodeEntry::new(vec![node.name], graph, ctx);
        let mut entry = ExternNodeEntry::new(entry, ty);

        // Step 2. hint variables with tensor graph
        entry.hint_variables(&mut node.tensor_graph)?;

        // Step 3. make a tensor graph
        entry.test_tensor_graph(&node.tensor_graph)?;
        for (_, n) in node.tensor_graph {
            entry.add_tensor_graph(n);
        }

        // Step 4. store
        Ok(entry.build())
    }
}

pub struct ExternTensorGraphCondition<'a> {
    pub nodes: &'a BTreeMap<u64, ast::GraphNode>,

    pub names: &'static [&'static str],
    pub ty_inputs: Option<ast::GraphInputsType>,
    // note: the args should be sorted
    pub args: Option<&'static [&'static str]>,
    pub is_sized: Option<bool>,
    pub repeatable: Option<bool>,
    pub is_id_zero: bool,
}

impl<'a> ExternTensorGraphCondition<'a> {
    pub fn test(self) -> Result<()> {
        // test the number of nodes
        if self.nodes.len() != self.names.len() {
            return GraphNodeError::MismatchedSize {
                expected: self.names,
                given: self.nodes.len(),
            }
            .into();
        }

        for (id, (name, node)) in self.names.iter().zip(self.nodes.values()).enumerate() {
            self.test_each_node(&[name], id as u64, node)?;
        }
        Ok(())
    }

    fn test_each_node(
        &self,
        names: &[&'static str],
        id: u64,
        node: &'a ast::GraphNode,
    ) -> Result<()> {
        // Step 1. test the number of calls (should be 1)
        {
            let given = node.calls.len();
            if given != 1 {
                return GraphCallError::MismatchedSize {
                    expected: names.to_vec(),
                    given,
                }
                .into();
            }
        }

        // Step 2. test the node id
        if self.is_id_zero && id != node.id {
            return GraphNodeError::MismatchedId {
                expected: id,
                given: node.id,
            }
            .into();
        }

        let call = &node.calls[0];
        let name = &call.name;

        // Step 3. test the name
        if !names.contains(&name.as_str()) {
            return GraphCallError::MismatchedName {
                expected: names.to_vec(),
                given: name.clone(),
            }
            .into();
        }

        // Step 4. test inputs
        if let Some(expected) = self.ty_inputs {
            let given = call.get_inputs_ty();
            if expected != given {
                return GraphCallError::MismatchedInputsType { expected, given }.into();
            }
        }

        // Step 5. test repeat
        if let Some(expected) = self.repeatable {
            let given = call.repeat.is_some();
            if expected != given {
                return GraphCallError::MismatchedRepeat { expected, given }.into();
            }
        }

        // Step 6. test the args
        if let Some(expected) = self.args {
            #[cfg(feature = "test-nightly")]
            {
                assert!(expected.is_sorted(), "the args should be sorted");
            }

            // note: the keywords are already sorted according to BTreeMap.
            let given = match &call.args {
                Some(args) => args.keys().collect(),
                None => vec![],
            };

            if given != expected {
                return GraphCallError::MismatchedArgs {
                    expected,
                    given: given.into_iter().cloned().collect(),
                }
                .into();
            }
        }

        // Step 7. test the size
        if let Some(expected) = self.is_sized {
            let given = node.shapes.is_some();
            if expected != given {
                return GraphNodeError::MismatchedShapesExistence { expected, given }.into();
            }
        }

        Ok(())
    }
}

struct ExecNodeEntry {
    name: String,
    graph: RefGraph,
    links: Vec<Vec<String>>,
}

impl ExecNodeEntry {
    fn try_new(
        name: String,
        graph: RefGraph,
        tensor_graph: BTreeMap<u64, ast::GraphNode>,
    ) -> Result<Self> {
        Ok(Self {
            name,
            graph,
            links: ExecNodeEntry::get_links(tensor_graph)?,
        })
    }

    fn get_links(tensor_graph: BTreeMap<u64, ast::GraphNode>) -> Result<Vec<Vec<String>>> {
        tensor_graph
            .into_iter()
            .enumerate()
            .map(|(expected, (given, node))| {
                let expected = expected as u64 + 1;

                // test id
                if expected != given {
                    return GraphNodeError::MismatchedId { expected, given }.into();
                }

                // test the shape
                if node.shapes.is_some() {
                    return GraphNodeError::UnexpectedShapes.into();
                }

                // test the calls
                if node.calls.is_empty() {
                    return GraphNodeError::EmptyCalls.into();
                }
                node.calls
                    .into_iter()
                    .map(|call| {
                        if call.inputs.is_some() {
                            return GraphCallError::UnexpectedInputs.into();
                        }
                        if call.args.is_some() {
                            return GraphCallError::UnexpectedArgs.into();
                        }
                        if call.repeat.is_some() {
                            return GraphCallError::UnexpectedRepeat.into();
                        }
                        Ok(call.name)
                    })
                    .collect()
            })
            .collect()
    }

    fn build(self) -> ExecIR {
        ExecIR {
            data: IRData::with_no_shapes(self.name, self.graph),
            links: self.links,
        }
    }
}

struct ExecFile(ast::File);
impl<'a> ASTBuild<'a> for ExecFile {
    type Args = ();
    type Output = ExecIR;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        let node = self.0.node;

        if !node.withs.is_empty() {
            return ExecBuildError::UnexpectedWiths.into();
        }
        if !node.children.is_empty() {
            return ExecBuildError::UnexpectedChildren.into();
        }
        if node.tensor_graph.is_empty() {
            return ExecBuildError::EmptyGraph.into();
        }

        // Step 1. make a graph
        let graph = Graph::try_with_variables(ctx.root.seed.generate(), node.graph, true)?.into();

        let entry = ExecNodeEntry::try_new(node.name, graph, node.tensor_graph)?;

        // Step 2. store
        Ok(entry.build())
    }
}
