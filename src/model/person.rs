use chrono::NaiveDateTime;

pub struct Person {
    _unique: i64,
    pub nickname: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Person {
    pub fn unique(&self) -> i64 {
        self._unique
    }

    pub fn nickname(&self) -> &String {
        &self.nickname
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn created_at(&self) -> u128 {
        self.created_at.and_utc().timestamp_millis() as u128
    }

    pub fn updated_at(&self) -> u128 {
        self.updated_at.and_utc().timestamp_millis() as u128
    }

    pub fn update_nickname(&mut self, nickname: String) {
        self.nickname = nickname
    }

    pub fn update_password(&mut self, password: String) {
        self.password = password
    }
}

mod sql {
    use rusqlite::OptionalExtension;

    use crate::consts::database::connection;
    use crate::consts::database::Result;

    impl crate::model::Model for super::Person {
        fn initialize() -> &'static str {
            "
                CREATE TABLE IF NOT EXISTS Person (
                    _unique      INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
                    nickname     TEXT      NOT NULL,
                    password     TEXT      NOT NULL,
                    created_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
                    updated_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TRIGGER IF NOT EXISTS update_Person_updated_at
                AFTER UPDATE ON Person
                FOR EACH ROW
                BEGIN
                    UPDATE Person
                    SET updated_at = CURRENT_TIMESTAMP
                    WHERE _unique = OLD._unique;
                END;
            "
        }
    }

    impl super::Person {
        pub fn insert_one(nickname: &String, password: &String) -> Result<Self> {
            let sql = "INSERT INTO Person (nickname, password) VALUES (?1, ?2) RETURNING _unique, nickname, password, created_at, updated_at";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let item = statement.query_row([nickname, password], |row| {
                Ok(Self {
                    _unique: row.get(0)?,
                    nickname: row.get(1)?,
                    password: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?;

            Ok(item)
        }

        pub fn select_one_by_unique(unique: i64) -> Result<Option<Self>> {
            let sql = "SELECT _unique, nickname, password, created_at, updated_at FROM Person WHERE _unique = ?1 LIMIT 1";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let item = statement
                .query_row([unique], |row| {
                    Ok(Self {
                        _unique: row.get(0)?,
                        nickname: row.get(1)?,
                        password: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })
                .optional()?;

            Ok(item)
        }

        pub fn select_one_by_nickname(nickname: &String) -> Result<Option<Self>> {
            let sql = "SELECT _unique, nickname, password, created_at, updated_at FROM Person WHERE _unique = ?1 LIMIT 1";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let item = statement
                .query_row([nickname], |row| {
                    Ok(Self {
                        _unique: row.get(0)?,
                        nickname: row.get(1)?,
                        password: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })
                .optional()?;

            Ok(item)
        }

        pub fn select_one_by_nickname_password(
            nickname: &String,
            password: &String,
        ) -> Result<Option<Self>> {
            let sql = "SELECT _unique, nickname, password, created_at, updated_at FROM Person WHERE nickname = ?1 AND password = ?2 LIMIT 1";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let item = statement
                .query_row([nickname, password], |row| {
                    Ok(Self {
                        _unique: row.get(0)?,
                        nickname: row.get(1)?,
                        password: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })
                .optional()?;

            Ok(item)
        }

        pub fn update_one_by_unique(unique: i64, item: &Self) -> Result<()> {
            let sql = "UPDATE Person SET nickname = ?2, password = ?3 WHERE _unique = ?1";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            statement.execute([&unique.to_string(), item.nickname(), item.password()])?;

            Ok(())
        }
    }
}
