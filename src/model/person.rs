use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Queryable, Selectable};

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
