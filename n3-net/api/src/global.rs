use std::sync::Mutex;

use lazy_static::lazy_static;

pub struct ExecRoot(Mutex<n3_builder::ExecRoot>);

unsafe impl Send for ExecRoot {}
unsafe impl Sync for ExecRoot {}

impl ExecRoot {
    pub fn with<T>(&self, f: impl FnOnce(&mut n3_builder::ExecRoot) -> T) -> T {
        let mut root = self.0.lock().unwrap();
        f(&mut *root)
    }
}

#[derive(Default)]
pub struct WorkRoot(Mutex<crate::model::WorkRoot>);

impl WorkRoot {
    pub fn get(&self, id: n3_machine_ffi::WorkId) -> Option<crate::model::Work> {
        self.0.lock().unwrap().get(id)
    }

    pub fn get_all(&self) -> Vec<crate::model::Work> {
        self.0.lock().unwrap().get_all()
    }

    pub fn delete(&self, id: n3_machine_ffi::WorkId) -> bool {
        self.0.lock().unwrap().delete(id)
    }
}

lazy_static! {
    pub static ref EXEC_ROOT: ExecRoot = {
        let envs = n3_builder::GlobalVars::default();
        let root = n3_builder::ExecRoot::try_new(envs).unwrap();
        ExecRoot(Mutex::new(root))
    };
    pub static ref WORK_ROOT: WorkRoot = Default::default();
}
