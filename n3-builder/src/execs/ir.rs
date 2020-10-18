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
use crate::variable::Link;

#[derive(Debug)]
pub struct ExecIR {
    pub data: IRData,
    pub links: Vec<Vec<String>>,
}

impl PartialEq for ExecIR {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl ExecIR {
    pub fn build(self, root: &NodeRoot, args: Vars) -> Result<Program> {
        // prune graph
        let mut nodes = BTreeMap::new();

        let graph = Rc::try_unwrap(self.data.graph)
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

        // link nodes
        for links in &self.links {
            // the calls should not be empty.
            let mut last = &nodes[&links[0]];

            for new in links.iter().skip(1).map(|x| &nodes[x]) {
                let last_shapes = last.get_output_shapes();
                let new_shapes = new.get_input_shapes();
                last_shapes.link_to(&new_shapes)?;

                last = new;
            }
        }

        // build nodes
        let nodes: BTreeMap<_, _> = nodes
            .into_iter()
            .map(|(k, v)| Ok((k, v.build(root)?)))
            .collect::<Result<_>>()?;

        // get extern scripts
        let mut scripts = BTreeMap::new();
        for node in nodes.values() {
            node.add_scripts(root, &mut scripts)?;
        }

        Ok(Program {
            graph,
            nodes,
            scripts,
        })
    }
}

impl CloneSafe for ExecIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self {
            data: self.data.clone_safe(seed, variables),
            links: self.links.clone(),
        }
    }
}
