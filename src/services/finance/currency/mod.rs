mod transaction;

use crate::model::finance::currency::numeric_code::NumericCode;

use crate::services::connection;

impl NumericCode {
    pub fn select_all() -> Option<Vec<Self>> {
        todo!()
    }

    pub fn select_one_by_code(code: i64) -> Option<Self> {
        todo!()
    }
}
