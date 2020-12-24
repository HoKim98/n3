use std::sync::Mutex;

use lazy_static::lazy_static;
use rocket_cors::AllowedOrigins;

use crate::db::Database;
use crate::error::Result;

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
    pub fn insert(&self, conn: &Database, work: &crate::model::Work) -> Result<crate::model::Work> {
        self.0.lock().unwrap().insert(conn, work)
    }

    pub fn get(&self, id: n3_machine_ffi::WorkId) -> Result<crate::model::Work> {
        self.0.lock().unwrap().get(id)
    }

    pub fn get_all(&self) -> Result<Vec<crate::model::Work>> {
        self.0.lock().unwrap().get_all()
    }

    pub fn delete(&self, id: n3_machine_ffi::WorkId) -> Result<()> {
        self.0.lock().unwrap().delete(id)
    }
}

lazy_static! {
    pub static ref EXEC_ROOT: ExecRoot = {
        let config = n3_builder::ExecRootConfig {
            create_root_dir: Some(true),
        };

        let envs = n3_builder::GlobalVars::default();
        let root = n3_builder::ExecRoot::try_new(envs, config).unwrap();
        ExecRoot(Mutex::new(root))
    };
    pub static ref WORK_ROOT: WorkRoot = Default::default();
}

pub fn allowed_origins() -> AllowedOrigins {
    AllowedOrigins::all()
}
