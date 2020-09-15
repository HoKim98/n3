use std::collections::BTreeMap;

use super::super::ir::NodeIR;
use super::super::root::NodeRoot;
use super::graph::GraphNodeEntry;
use crate::ast;
use crate::cache::CloneSafe;
use crate::error::{BuildError, Result};
use crate::graph::{Graph, RefGraph};
use crate::tensor::TensorNode;

pub trait ASTBuild<'a> {
    type Args;
    type Output;

    fn build(self, ctx: &mut Context<'a>, args: Self::Args) -> Result<Self::Output>;
}

pub struct Context<'a> {
    root: &'a NodeRoot,
    parent: BTreeMap<NodeName, RefGraph>,
    uses: BTreeMap<String, TensorNode>,
}

impl<'a> Context<'a> {
    pub fn new(root: &'a NodeRoot) -> Self {
        Context {
            root,
            parent: Default::default(),
            uses: Default::default(),
        }
    }
}

type NodeName = Vec<String>;

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
        self.children.insert(node.name.clone(), node.into());
        Ok(())
    }

    fn add_tensor_graph(&mut self, id: u64, node: ast::GraphNode) -> Result<()> {
        let last_id = self.last_tensor_id;
        if id < last_id || id - last_id != 1 && !(last_id == 0 && id == 0) {
            Err(BuildError::MismatchedGraphNodeId {
                expected: last_id + 1,
                given: id,
            }
            .into())
        } else {
            GraphNodeEntry {
                root: self,
                id,
                node,
            }
            .build()
        }
    }

    fn add_tensor_graph_input(&mut self, node: ast::GraphNode) -> Result<()> {
        todo!()
    }

    fn add_tensor_graph_default(&mut self, id: u64, node: ast::GraphNode) -> Result<()> {
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

impl<'a> Context<'a> {
    fn get(&mut self, name: &str) -> Result<TensorNode> {
        if let Some(node) = self.uses.get(name) {
            let mut variables = vec![];
            Ok(node.clone_safe(&self.root.seed, &mut variables))
        } else {
            Ok(self.root.get(name)?.into())
        }
    }
}

impl<'a> ASTBuild<'a> for ast::File {
    type Args = NodeName;
    type Output = NodeIR;

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
        ctx.parent.insert(name.clone(), graph.clone());

        let mut entry = NodeEntry {
            name,
            graph,
            children: Default::default(),
            ctx,
            last_tensor_id: 0,
        };

        // Step 2. import remote models
        for (name, u) in self.uses {
            entry.add_use(name, u)?;
        }

        // Step 3. hint variables with tensor graph
        {
            let graph = entry.graph.borrow();
            for (&id, n) in node.tensor_graph.iter_mut() {
                if let Some(shapes) = &mut n.shapes {
                    for (x, shape) in &mut shapes.0 {
                        if let Some(shape) = shape {
                            let out = ast::Out::new(id, x.clone());
                            *shape = graph.hint(&out, shape)?;
                        }
                    }
                }
            }
        }

        // Step 4. re-define nodes (with)
        for (name, w) in node.withs {
            entry.add_with(name, w)?;
        }

        // Step 5. build children nodes
        for (name, child) in node.children {
            entry.add_child(name, child)?;
        }

        // Step 6. make a tensor graph
        for (id, n) in node.tensor_graph {
            entry.add_tensor_graph(id, n)?;
        }

        todo!()
    }
}

struct ExternFile(ast::File);
impl<'a> ASTBuild<'a> for ExternFile {
    type Args = ();
    type Output = NodeIR;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        let file = self.0;

        dbg!(&file);
        todo!()
    }
}

struct ExecFile(ast::File);
impl<'a> ASTBuild<'a> for ExecFile {
    type Args = ();
    type Output = NodeIR;

    fn build(self, ctx: &mut Context<'a>, (): Self::Args) -> Result<Self::Output> {
        todo!()
    }
}
