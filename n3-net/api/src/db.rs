use rocket_contrib::databases::diesel;

#[database("n3_net_api")]
pub struct Database(diesel::SqliteConnection);
