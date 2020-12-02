use diesel::prelude::*;

use super::table::TableId;
use crate::schema::machines::{self, table};

use self::machines as table_mod;

#[table_name = "machines"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Machine {
    pub id: Option<TableId>,
    pub provider: Option<String>,
    pub domain: Option<String>,
    pub device: Option<String>,
    pub device_id: Option<String>,
}
crate::impl_table_mut!(Machine, TableId);

impl From<n3_machine_ffi::Query> for Machine {
    fn from(x: n3_machine_ffi::Query) -> Self {
        Self {
            id: None,
            provider: x.provider,
            domain: x.domain,
            device: x.device,
            device_id: x.id,
        }
    }
}

impl From<Machine> for n3_machine_ffi::Query {
    fn from(x: Machine) -> Self {
        Self {
            provider: x.provider,
            domain: x.domain,
            device: x.device,
            id: x.device_id,
        }
    }
}
