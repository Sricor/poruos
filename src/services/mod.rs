pub mod finance;
pub mod person;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sqlite::SqliteConnection;

fn connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    crate::consts::database::DATABASE.get().unwrap()
}
