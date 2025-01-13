use std::sync::LazyLock;

pub mod database {
    use r2d2::{Pool, PooledConnection};
    use r2d2_sqlite::SqliteConnectionManager;

    use super::LazyLock;

    pub static DATABASE: LazyLock<Pool<SqliteConnectionManager>> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        let path = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = SqliteConnectionManager::file(path);
        let pool = r2d2::Pool::new(database).unwrap();

        let conn = pool.get().unwrap();
        conn.execute_batch(&crate::model::initialize()).unwrap();

        pool
    });

    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub fn connection() -> Result<PooledConnection<SqliteConnectionManager>> {
        use crate::consts::database::DATABASE;

        Ok(DATABASE.get()?)
    }
}

pub mod claim_encrypt {
    use crate::common::cipher::ChaCha20Poly1305;

    use super::LazyLock;

    pub static ENCRYPTER: LazyLock<ChaCha20Poly1305> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        ChaCha20Poly1305::new(
            std::env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set")
                .as_bytes()
                .try_into()
                .unwrap(),
            std::env::var("SECRET_NONCE")
                .expect("SECRET_NONCE must be set")
                .as_bytes()
                .try_into()
                .unwrap(),
        )
        .unwrap()
    });
}
