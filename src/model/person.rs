use chrono::NaiveDateTime;

pub struct Person {
    _unique: i64,
    nickname: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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
    const TABLE: &str = "
        CREATE TABLE IF NOT EXISTS person (
            _unique      INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
            nickname     TEXT      NOT NULL,
            password     TEXT      NOT NULL,
            created_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
            updated_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TRIGGER IF NOT EXISTS update_person_updated_at
        AFTER UPDATE ON person
        FOR EACH ROW
        BEGIN
            UPDATE person
            SET updated_at = CURRENT_TIMESTAMP
            WHERE _unique = OLD._unique;
        END;
    ";
}
