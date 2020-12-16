pub type TableId = i32;

#[macro_export]
macro_rules! impl_table_mut {
    ($ty:ty, $id:ty) => {
        use crate::db::Database;
        use crate::error::{Error, Result};

        impl $ty {
            pub fn insert(&self, conn: &Database) -> Result<Self> {
                diesel::insert_into(table).values(self).execute(&**conn)?;

                table
                    .order(table_mod::id.desc())
                    .first(&**conn)
                    .map_err(|e| e.into())
            }

            pub fn get(conn: &Database, id: $id) -> Result<Self> {
                table.find(id).first(&**conn).map_err(|e| e.into())
            }

            pub fn get_all(conn: &Database) -> Result<Vec<Self>> {
                table
                    .order(table_mod::id)
                    .load(&**conn)
                    .map_err(|e| e.into())
            }

            pub fn update(&self, conn: &Database) -> Result<()> {
                match self.id {
                    Some(id) => {
                        diesel::update(table.find(id)).set(self).execute(&**conn)?;
                        Ok(())
                    }
                    None => Err(Error::RequireAKey),
                }
            }

            pub fn delete(conn: &Database, id: $id) -> Result<()> {
                diesel::delete(table.find(id)).execute(&**conn)?;
                Ok(())
            }
        }
    };
}
