use std::collections::BTreeMap;

use inflector::Inflector;

use n3_builder::inflector;
use n3_machine_ffi::{Query, WorkId, WorkStatus};

use super::machines::Machine;
use super::table::TableId;
use crate::db::Database;
use crate::error::{Error, Result};
use crate::global::EXEC_ROOT;

#[derive(Default)]
pub struct WorkRoot(BTreeMap<WorkId, WorkEntry>);

pub struct WorkEntry {
    info: Work,
    inner: n3_net_client::Work,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Work {
    id: Option<String>,
    command: String,
    exec: String,
    variables: BTreeMap<String, String>,
    machines: Vec<TableId>,

    status: Option<WorkStatus>,
}

impl WorkRoot {
    pub fn insert(&mut self, conn: &Database, work: &Work) -> Result<Work> {
        let work = work.try_insert(conn)?;
        let info = work.info.clone();

        self.0.insert(work.id(), work);
        Ok(info)
    }

    pub fn get(&mut self, id: WorkId) -> Result<Work> {
        match self.0.get_mut(&id) {
            Some(entry) => Self::update_status(entry),
            None => Err(Error::NoSuchWork { id }),
        }
    }

    pub fn get_all(&mut self) -> Result<Vec<Work>> {
        self.0.values_mut().map(Self::update_status).collect()
    }

    fn update_status(entry: &mut WorkEntry) -> Result<Work> {
        entry.info.status = Some(entry.inner.status()?);
        Ok(entry.info.clone())
    }

    pub fn delete(&mut self, id: WorkId) -> Result<()> {
        match self.0.remove(&id) {
            Some(mut entry) => entry.inner.terminate().map_err(|e| e.into()),
            None => Err(Error::NoSuchWork { id }),
        }
    }
}

impl WorkEntry {
    fn id(&self) -> WorkId {
        self.inner.id()
    }
}

impl Work {
    fn try_insert(&self, conn: &Database) -> Result<WorkEntry> {
        // Step 1. parse the machines
        let machines: Vec<_> = self
            .machines
            .iter()
            .map(|&id| {
                Machine::get(conn, id)
                    .map(Query::from)
                    .map(|x| x.to_string())
            })
            .collect::<Result<_>>()?;

        // Step 2. build a program
        let program = EXEC_ROOT.with(|root| {
            let args = root.get(&self.exec.to_pascal_case())?;

            for (name, value) in &self.variables {
                args.set(name, value)?;
            }

            args.build_with_env()
        })?;

        // Step 3. spawn a work
        let work = n3_net_client::Work::spawn(&program, &self.command, &machines)?;

        // Step 4. finalize
        let mut info = self.clone();
        info.id = Some(work.id().to_string());

        Ok(WorkEntry { info, inner: work })
    }
}
