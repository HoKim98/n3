use rocket_contrib::json::Json;

use crate::model::*;

type Id = TableId;
type Table = Machine;

#[post("/machine", data = "<obj>")]
pub fn insert(conn: crate::db::Database, obj: Json<Table>) -> Json<ObjResult<Table>> {
    Json(obj.insert(&conn).into())
}

#[get("/machine/<id>")]
pub fn get(conn: crate::db::Database, id: Id) -> Json<ObjResult<Table>> {
    Json(Table::get(&conn, id).into())
}

#[get("/machine")]
pub fn get_all(conn: crate::db::Database) -> Json<ObjResult<Vec<Table>>> {
    Json(Table::get_all(&conn).into())
}

#[put("/machine/<id>", data = "<obj>")]
pub fn update(conn: crate::db::Database, id: Id, mut obj: Json<Table>) -> Json<BoolResult> {
    obj.id = Some(id);
    Json(obj.update(&conn).into())
}

#[delete("/machine/<id>")]
pub fn delete(conn: crate::db::Database, id: Id) -> Json<BoolResult> {
    Json(Table::delete(&conn, id).into())
}
