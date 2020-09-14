use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

use crate::ast;
use crate::error::Result;
use crate::nodes::NodeRoot;
use crate::seed::Seed;

pub struct NodeCache<T: Build> {
    paths: RefCell<HashMap<String, String>>,
    caches_source: RefCell<HashMap<String, String>>,
    caches: RefCell<HashMap<String, T>>,
}

pub trait Build: CloneSafe {
    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self>
    where
        Self: Sized;
}

pub trait CloneSafe {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self;
}

impl<T: Build> NodeCache<T> {
    pub fn new(caches: HashMap<String, String>) -> Self {
        Self {
            paths: RefCell::default(),
            caches_source: RefCell::new(caches),
            caches: RefCell::default(),
        }
    }

    pub fn get(&self, name: &str, root: &NodeRoot) -> Result<T> {
        if let Some(cache) = self.caches.borrow().get(name) {
            let mut variables = vec![];
            return Ok(cache.clone_safe(&root.seed, &mut variables));
        }

        if let Some(path) = self.paths.borrow_mut().remove(name) {
            let source = fs::read_to_string(path)?;
            return self.build_and_store(name, root, source);
        }
        if let Some(source) = self.caches_source.borrow_mut().remove(name) {
            return self.build_and_store(name, root, source);
        }
        todo!()
    }

    fn build_and_store(&self, name: &str, root: &NodeRoot, source: String) -> Result<T> {
        // TODO: detect cycling
        let object = T::build(root, name, source)?;

        let mut variables = vec![];
        let cache = object.clone_safe(&root.seed, &mut variables);

        self.caches.borrow_mut().insert(name.to_string(), cache);
        Ok(object)
    }
}
