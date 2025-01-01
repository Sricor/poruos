mod transaction;

use diesel::prelude::*;

use crate::model::finance::currency::NumericCode;
use crate::schema::finance_currency_numeric_code;

use crate::services::connection;

impl NumericCode {
    pub fn select_all() -> Option<Vec<Self>> {
        let conn = &mut connection();

        finance_currency_numeric_code::table.load::<Self>(conn).ok()
    }

    pub fn select_one_by_code(code: i64) -> Option<Self> {
        let conn = &mut connection();

        finance_currency_numeric_code::table
            .filter(finance_currency_numeric_code::code.eq(code))
            .first(conn)
            .ok()
    }
}
