use crate::model::finance::currency::transaction::Transaction;
use crate::model::finance::Amount;

use super::connection;

impl Transaction {
    pub fn insert_one(
        owner: i64,
        amount: Amount,
        numeric_code: i64,
        remarks: Option<&String>,
    ) -> Option<Self> {
        todo!()
    }

    pub fn select_one_by_unique_owner(id: i64, owner: i64) -> Option<Self> {
        todo!()
    }

    pub fn select_by_owner(owner: i64, limit: i64, offset: i64) -> Option<Vec<Self>> {
        todo!()
    }
}
