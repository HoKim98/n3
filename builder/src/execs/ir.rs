use std::collections::BTreeMap;
use std::rc::Rc;

use super::program::Program;
use super::var::Vars;
use crate::ast;
use crate::context::CloneSafe;
use crate::error::{ExecBuildError, Result};
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::IRData;

#[derive(Debug)]
pub struct ExecIR {
    pub data: IRData,
}

impl ExecIR {
    pub fn build(self, root: &NodeRoot, args: Vars) -> Result<Program> {
        let data = self.data;

        let mut nodes = BTreeMap::new();

        // prune graph
        let graph: BTreeMap<_, _> = Rc::try_unwrap(data.graph)
            .unwrap()
            .into_inner()
            .into_variables()
            .into_iter()
            .filter_map(|(var_name, var)| {
                let var_ref = var.borrow();

                if let Some(ast::LetType::Node(Some(ty))) = var_ref.ty {
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
                } else {
                    drop(var_ref);
                    Some(Ok((var_name, var)))
                }
            })
            .collect::<Result<_>>()?;

        dbg!(graph);
        todo!()
    }
}

impl CloneSafe for ExecIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self {
            data: self.data.clone_safe(seed, variables),
        }
    }
}
