use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Queryable, Selectable};

#[derive(Queryable, Selectable, AsChangeset)]
// #[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DepositWallet {
    _unique: i32,
    owner: i32,

    // nano-dollars
    balance: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub enum TransactionType {
    Revenue = 0,
    Expenditure = 1
}

// https://en.wikipedia.org/wiki/ISO_4217
pub enum NumericCode {
    CNY = 156,
    JPY = 392,
    USD = 840,
    EUR = 978,
}

pub struct Transaction {
    _unique: i32,
    wallet: i32,
    amount: i32,
    numeric_code: NumericCode,
    transaction_type: TransactionType,
    remarks: Option<String>,
    occurrence_at: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}