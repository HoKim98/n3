use rocket_contrib::json::Json;

use crate::global::WORK_ROOT;
use crate::model::*;

type Id = n3_machine_ffi::WorkId;
type Table = Work;

#[post("/work", data = "<obj>")]
pub fn insert(conn: crate::db::Database, obj: Json<Table>) -> Json<ObjResult<Table>> {
    Json(WORK_ROOT.insert(&conn, &*obj).into())
}

#[get("/work/<id>")]
pub fn get(id: Id) -> Json<ObjResult<Table>> {
    Json(WORK_ROOT.get(id).into())
}

#[get("/work")]
pub fn get_all() -> Json<ObjResult<Vec<Table>>> {
    Json(WORK_ROOT.get_all().into())
}

#[delete("/work/<id>")]
pub fn delete(id: Id) -> Json<BoolResult> {
    Json(WORK_ROOT.delete(id).into())
}
