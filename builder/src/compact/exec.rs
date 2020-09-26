use serde::{Deserialize, Serialize};

use super::graph::Graphs;
use super::ir_extern::Scripts;
use super::tensor::TensorNodes;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Program {
    pub(super) graphs: Graphs,
    pub(super) nodes: TensorNodes,
    pub(super) scripts: Scripts,
}
