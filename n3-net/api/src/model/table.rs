pub type TableId = i32;

#[macro_export]
macro_rules! impl_table_mut {
    ($ty:ty, $id:ty) => {
        use crate::db::Database;

        impl $ty {
            pub fn insert(&self, conn: &Database) -> Option<Self> {
                diesel::insert_into(table)
                    .values(self)
                    .execute(&**conn)
                    .unwrap();

                table.order(table_mod::id.desc()).first(&**conn).ok()
            }

            pub fn get(conn: &Database, id: $id) -> Option<Self> {
                table.find(id).first(&**conn).ok()
            }

            pub fn get_all(conn: &Database) -> Vec<Self> {
                table.order(table_mod::id).load(&**conn).unwrap_or_default()
            }

            pub fn update(&self, conn: &Database) -> bool {
                match self.id {
                    Some(id) => diesel::update(table.find(id))
                        .set(self)
                        .execute(&**conn)
                        .is_ok(),
                    None => false,
                }
            }

            pub fn delete(conn: &Database, id: $id) -> bool {
                diesel::delete(table.find(id)).execute(&**conn).is_ok()
            }
        }
    };
}
