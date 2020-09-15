use std::collections::BTreeMap;

use super::super::ir::NodeIR;
use super::graph::GraphNodeEntry;
use crate::ast;
use crate::context::{CloneSafe, Context, NodeName};
use crate::error::{BuildError, Result};
use crate::externs::ExternIR;
use crate::graph::{Graph, RefGraph};
use crate::tensor::TensorNode;

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
    graph: RefGraph,
    ctx: &'b mut Context<'a>,

    children: BTreeMap<String, TensorNode>,

    last_tensor_id: u64,
}

impl<'a, 'b> NodeEntry<'a, 'b> {
    fn new(name: NodeName, graph: RefGraph, ctx: &'b mut Context<'a>) -> Self {
        Self {
            name,
            graph,
            children: Default::default(),
            ctx,
            last_tensor_id: 0,
        }
    }

    fn hint_variables(&mut self, tensor_graph: &mut BTreeMap<u64, ast::GraphNode>) -> Result<()> {
        let graph = self.graph.borrow();
        for (&id, n) in tensor_graph.iter_mut() {
            if let Some(shapes) = &mut n.shapes {
                for (x, shape) in &mut shapes.0 {
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
        let mut node = self.get(&name)?; // TODO: 여기부터 만들기

        // Step 2. apply variables
        let args = {
            let graph = self.graph.borrow();
            with.graph
                .into_iter()
                .map(|(k, v)| {
                    let v = graph.replace_to(v)?;
                    let var = ast::Variable::with_name_value(k.clone(), Some(v));
                    Ok((k, var.into()))
                })
                .collect::<Result<_>>()?
        };
        node.apply_variables(args)?;

        // Step 3. store
        self.children.insert(name, node);
        Ok(())
    }

    fn add_child(&mut self, name: String, child: ast::Node) -> Result<()> {
        // Step 1. convert to file
        let file = ast::File {
            uses: Default::default(),
            node: child,
        };

        // Step 2. build
        let node = file.build(self.ctx, self.name.clone())?;

        // Step 3. store
        self.children.insert(node.name().to_string(), node.into());
        Ok(())
    }

    fn add_tensor_graph(&mut self, node: ast::GraphNode) -> Result<()> {
        let last_id = self.last_tensor_id;
        if node.id < last_id || node.id - last_id != 1 && !(last_id == 0 && node.id == 0) {
            Err(BuildError::MismatchedGraphNodeId {
                expected: last_id + 1,
                given: node.id,
            }
            .into())
        } else {
            GraphNodeEntry {
                root: self,
                id: node.id,
                node,
            }
            .build()
        }
    }

    fn build(self) -> NodeIR {
        todo!()
    }

    pub fn get(&mut self, name: &str) -> Result<TensorNode> {
        if let Some(node) = self.children.get(name) {
            let mut variables = vec![];
            Ok(node.clone_safe(&self.ctx.root.seed, &mut variables))
        } else {
            self.ctx.get(name)
        }
    }
}

impl<'a> ASTBuild<'a> for ast::File {
    type Args = NodeName;
    type Output = TensorNode;

    fn build(self, ctx: &mut Context<'a>, parent: Self::Args) -> Result<Self::Output> {
        if self.node.ty.is_extern() {
            return ExternFile(self).build(ctx, ());
        }
        if self.node.ty.is_exec() {
            return ExecFile(self).build(ctx, ());
        }

        let mut node = self.node;

        let mut name = parent.clone();
        name.push(node.name.clone());

        // Step 1. make a graph
        let graph: RefGraph =
            Graph::try_with_variables(ctx.root.seed.generate(), node.graph)?.into();
        ctx.add_child(name.clone(), graph.clone());

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
        for (name, child) in node.children {
            entry.add_child(name, child)?;
        }

        // Step 6. make a tensor graph
        for (_, n) in node.tensor_graph {
            entry.add_tensor_graph(n)?;
        }

        // Step 7. store
        Ok(entry.build().into())
    }
}

struct ExternNodeEntry<'a, 'b>(NodeEntry<'a, 'b>);
impl<'a, 'b> ExternNodeEntry<'a, 'b> {
    fn add_tensor_graph(&mut self, node: ast::GraphNode) -> Result<()> {
        dbg!(node); // TODO: 여기부터 시작
        todo!()
    }

    fn build(self) -> ExternIR {
        todo!()
    }
}

struct ExternFile(ast::File);
impl<'a> ASTBuild<'a> for ExternFile {
    type Args = ();
    type Output = TensorNode;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        let file = self.0;
        let mut node = file.node;

        let ty = node.ty.unwrap_extern();

        // Step 1. make a graph
        let graph: RefGraph =
            Graph::try_with_variables(ctx.root.seed.generate(), node.graph)?.into();

        let entry = NodeEntry::new(vec![node.name], graph, ctx);
        let mut entry = ExternNodeEntry(entry);

        // Step 2. hint variables with tensor graph
        entry.0.hint_variables(&mut node.tensor_graph)?;

        // Step 3. make a tensor graph
        let tensor_graph_size = node.tensor_graph.len();
        let expected_tensor_graph: &[_] = match ty {
            ast::ExternNodeType::Default => &["Input", "Output"],
            ast::ExternNodeType::Data => &["Output"],
            ast::ExternNodeType::Optim => &[],
        };
        assert_extern_tensor_graph_size(tensor_graph_size, expected_tensor_graph)?;

        for (_, n) in node.tensor_graph {
            entry.add_tensor_graph(n)?;
        }

        // Step 4. store
        Ok(entry.build().into())
    }
}

fn assert_extern_tensor_graph_size(given: usize, expected: &'static [&'static str]) -> Result<()> {
    if expected.len() == given {
        Ok(())
    } else {
        Err(BuildError::MismatchedGraphNodeSize {
            expected,
            given: given as u64,
        }
        .into())
    }
}

struct ExecFile(ast::File);
impl<'a> ASTBuild<'a> for ExecFile {
    type Args = ();
    type Output = TensorNode;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        todo!()
    }
}
