mod category;

use chrono::NaiveDateTime;

use crate::model::finance::Amount;

pub struct Transaction {
    _unique: i64,
    pub owner: i64,
    pub amount: Amount,
    pub numeric_code: i64,
    pub remarks: Option<String>,
    pub is_publish: bool,
    pub occurrence_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Transaction {
    pub fn unique(&self) -> i64 {
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


mod sql {
    const TABLE: &str = "
        CREATE TABLE IF NOT EXISTS finance_currency_transaction (
            _unique         INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
            owner           INTEGER   NOT NULL,
            amount          INTEGER   NOT NULL,
            numeric_code    INTEGER   NOT NULL,
            remarks         TEXT,
            is_publish      BOOLEAN   NOT NULL  DEFAULT TRUE,
            occurrence_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
            created_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
            updated_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(owner) REFERENCES person(_unique),
            FOREIGN KEY(numeric_code) REFERENCES finance_currency_numeric_code(code)
        );

        CREATE TRIGGER IF NOT EXISTS update_finance_currency_transaction_updated_at
        AFTER UPDATE ON finance_currency_transaction
        FOR EACH ROW
        BEGIN
            UPDATE finance_currency_transaction
            SET updated_at = CURRENT_TIMESTAMP
            WHERE _unique = OLD._unique;
        END;
    ";
}