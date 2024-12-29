use diesel::prelude::*;

use crate::model::finance::currency::transaction::Transaction;
use crate::schema::finance_currency_transaction;

use super::connection;

impl Transaction {
    pub fn insert_one(
        owner: i32,
        amount: i32,
        numeric_code: i32,
        remarks: Option<&String>,
    ) -> Option<Self> {
        let conn = &mut connection();

        let records = (
            finance_currency_transaction::owner.eq(owner),
            finance_currency_transaction::amount.eq(amount),
            finance_currency_transaction::numeric_code.eq(numeric_code),
            finance_currency_transaction::remarks.eq(remarks),
        );

        diesel::insert_into(finance_currency_transaction::table)
            .values(records)
            .returning(Self::as_returning())
            .get_result(conn)
            .ok()
    }

    pub fn select_one_by_unique_owner(id: i32, owner: i32) -> Option<Self> {
        let conn = &mut connection();

        finance_currency_transaction::table
            .find(id)
            .filter(finance_currency_transaction::owner.eq(owner))
            .first(conn)
            .ok()
    }

    pub fn select_by_owner(owner: i32, limit: i64, offset: i64) -> Option<Vec<Self>> {
        let conn = &mut connection();

        finance_currency_transaction::table
            .filter(finance_currency_transaction::owner.eq(owner))
            .limit(limit)
            .offset(offset)
            .get_results(conn)
            .ok()
    }
}
