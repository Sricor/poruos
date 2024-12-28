use std::sync::LazyLock;

pub mod database {
    use diesel::r2d2::ConnectionManager;
    use diesel::sqlite::SqliteConnection;

    use super::LazyLock;

    type Pool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

    pub static DATABASE: LazyLock<Pool> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    });
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
