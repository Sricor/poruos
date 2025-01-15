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
    use chrono::NaiveDateTime;
    use rusqlite::ToSql;

    use crate::consts::database::connection;
    use crate::consts::database::Result;
    use crate::model::finance::Amount;

    impl crate::model::Model for super::Transaction {
        fn initialize() -> &'static str {
            "
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
            "
        }
    }

    impl super::Transaction {
        pub fn insert_one(
            owner: i64,
            amount: Amount,
            numeric_code: i64,
            remarks: Option<&String>,
            occurrence_at: NaiveDateTime,
        ) -> Result<Self> {
            let sql = "
                INSERT INTO finance_currency_transaction (  
                    owner,
                    amount,
                    numeric_code,
                    remarks,
                    occurrence_at
                ) VALUES (?1, ?2, ?3, ?4, ?5) RETURNING 
                    _unique,
                    owner,
                    amount,
                    numeric_code,
                    remarks,
                    is_publish,
                    occurrence_at, 
                    created_at, 
                    updated_at
            ";

            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let owner = owner.to_sql()?;
            let amount = amount.to_sql()?;
            let numeric_code = numeric_code.to_sql()?;
            let remarks = remarks.to_sql().unwrap_or("NULL".into());
            let occurrence_at = occurrence_at.to_sql()?;
            let item = statement.query_row(
                [owner, amount, numeric_code, remarks, occurrence_at],
                |row| {
                    Ok(Self {
                        _unique: row.get(0)?,
                        owner: row.get(1)?,
                        amount: row.get(2)?,
                        numeric_code: row.get(3)?,
                        remarks: row.get(4)?,
                        is_publish: row.get(5)?,
                        occurrence_at: row.get(6)?,
                        created_at: row.get(7)?,
                        updated_at: row.get(8)?,
                    })
                },
            )?;

            Ok(item)
        }

        pub fn select_one_by_unique_owner(id: i64, owner: i64) -> Option<Self> {
            todo!()
        }

        pub fn select_by_owner(owner: i64, limit: i64, offset: i64) -> Option<Vec<Self>> {
            todo!()
        }
    }
}
