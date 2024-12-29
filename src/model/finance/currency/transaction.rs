use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Queryable, Selectable};

#[derive(Queryable)]
#[diesel(table_name = crate::schema::finance_currency_transaction_category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionCategory {
    _unique: i32,
    pub owner: i32,
    pub remarks: Option<String>,
    pub is_publish: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::finance_currency_transaction)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    _unique: i32,
    pub owner: i32,
    pub amount: i32,
    pub numeric_code: i32,
    pub remarks: Option<String>,
    pub is_publish: bool,
    pub occurrence_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Transaction {
    pub fn unique(&self) -> i32 {
        self._unique
    }

    pub fn occurrence_at(&self) -> u128 {
        self.occurrence_at.and_utc().timestamp_millis() as u128
    }

    pub fn created_at(&self) -> u128 {
        self.created_at.and_utc().timestamp_millis() as u128
    }

    pub fn updated_at(&self) -> u128 {
        self.updated_at.and_utc().timestamp_millis() as u128
    }
}
