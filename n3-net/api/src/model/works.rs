use std::collections::BTreeMap;

use n3_machine_ffi::{Query, WorkId, WorkStatus};

use crate::db::Database;
use crate::error::Result;
use crate::global::EXEC_ROOT;

#[derive(Default)]
pub struct WorkRoot(BTreeMap<WorkId, WorkEntry>);

pub struct WorkEntry {
    info: Work,
    inner: n3_net_client::Work,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Work {
    id: Option<WorkId>,
    command: String,
    exec: String,
    variables: BTreeMap<String, String>,

    status: Option<WorkStatus>,
}

impl WorkRoot {
    pub fn get(&mut self, id: WorkId) -> Option<Work> {
        Self::update_status(self.0.get_mut(&id)?)
    }

    pub fn get_all(&mut self) -> Vec<Work> {
        self.0
            .values_mut()
            .filter_map(Self::update_status)
            .collect()
    }

    fn update_status(entry: &mut WorkEntry) -> Option<Work> {
        entry.info.status = Some(entry.inner.status().ok()?);
        Some(entry.info.clone())
    }

    pub fn delete(&self, id: WorkId) -> bool {
        todo!()
    }
}

impl Work {
    pub fn insert(&self, conn: &Database) -> Option<Self> {
        self.try_insert(conn).ok()
    }

    fn try_insert(&self, conn: &Database) -> Result<Self> {
        // Step 1. parse the machines
        let machines: Vec<_> = super::machines::Machine::get_all(conn)
            .into_iter()
            .map(Query::from)
            .map(|x| format!("{}", x))
            .collect();

        // Step 2. build a program
        let program = EXEC_ROOT.with(|root| {
            let args = root.get(&self.exec)?;
            for (name, value) in &self.variables {
                args.set(name, value)?;
            }

            args.build_with_env()
        })?;

        // Step 3. spawn a work
        let work = n3_net_client::Work::spawn(&program, &self.command, &machines)?;

        // Step 4. finalize
        let mut obj = self.clone();
        obj.id = Some(work.id());
        Ok(obj)
    }
}
