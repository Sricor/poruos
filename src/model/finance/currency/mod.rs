pub mod transaction;

use chrono::NaiveDateTime;
use diesel::prelude::Queryable;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::finance_currency_numeric_code)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NumericCode {
    _unique: i64,
    code: i64,
    symbol: String,
    _created_at: NaiveDateTime,
    _updated_at: NaiveDateTime,
}

impl NumericCode {
    pub fn code(&self) -> i64 {
        self.code
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }
}
