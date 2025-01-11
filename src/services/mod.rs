use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub mod finance;
pub mod person;

fn connection() -> PooledConnection<SqliteConnectionManager> {
    use crate::consts::database::DATABASE;

    DATABASE.get().unwrap()
}
