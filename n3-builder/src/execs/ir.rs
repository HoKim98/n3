use std::collections::BTreeMap;
use std::rc::Rc;

use super::program::{Nodes, Program};
use super::var::Vars;
use crate::ast;
use crate::context::CloneSafe;
use crate::error::{ExecBuildError, Result};
use crate::graph::{RefGraph, Table};
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::IRData;

#[derive(Debug, PartialEq)]
pub struct ExecIR {
    pub data: IRData,
}

impl ExecIR {
    pub fn build(self, root: &NodeRoot, args: Vars) -> Result<Program> {
        let data = self.data;

        // prune graph
        let (graph, nodes) = prune_graph(root, data.graph, args)?;

        Ok(Program { graph, nodes })
    }
}

impl CloneSafe for ExecIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self {
            data: self.data.clone_safe(seed, variables),
        }
    }
}

fn prune_graph(root: &NodeRoot, graph: RefGraph, args: Vars) -> Result<(Table, Nodes)> {
    let mut nodes = BTreeMap::new();

    let graph = Rc::try_unwrap(graph)
        .unwrap()
        .into_inner()
        .into_variables()
        .into_iter()
        .filter_map(|(var_name, var)| {
            let mut var_ref = var.borrow_mut();
            let ty = var_ref.ty.as_ref().unwrap();

            // prune the nodes
            if let ast::LetType::Node(Some(ty)) = ty {
                let ty = *ty;
                let name = match &var_ref.value.as_ref().and_then(|x| x.unwrap_node_name()) {
                    Some(name) => name.to_string(),
                    None => match args.get_node_name(&var_name, ty) {
                        Ok(x) => x,
                        Err(e) => return Some(Err(e)),
                    },
                };

                let node = match root.get(&name) {
                    Ok(x) => x,
                    Err(e) => return Some(Err(e)),
                };

                if node.ty != ty {
                    // the normal extern node can be applied into normal node.
                    if !(node.ty == ast::LetNodeType::Extern(ast::ExternNodeType::Default)
                        && ty == ast::LetNodeType::Default)
                    {
                        return Some(
                            ExecBuildError::MismatchedNodeType {
                                expected: ty,
                                given: node.ty,
                            }
                            .into(),
                        );
                    }
                }

                nodes.insert(var_name, node);
                None
            }
            // otherwise, update the variables
            else {
                match args.try_get_checked(&var_name, ty.clone()) {
                    Ok(Some(value)) => {
                        let value = value.borrow().value.clone();
                        var_ref.value = value;
                    }
                    Ok(None) => {}
                    Err(e) => return Some(Err(e)),
                };

                drop(var_ref);
                Some(Ok((var_name, var)))
            }
        })
        .collect::<Result<_>>()?;

    Ok((graph, nodes))
}
