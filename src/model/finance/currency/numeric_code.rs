pub struct NumericCode {
    pub code: i64,
    pub symbol: String,
}

impl NumericCode {
    pub fn code(&self) -> i64 {
        self.code
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }
}

mod sql {
    use rusqlite::OptionalExtension;

    use crate::consts::database::connection;
    use crate::consts::database::Result;

    impl crate::model::Model for super::NumericCode {
        fn initialize() -> &'static str {
            "
                CREATE TABLE IF NOT EXISTS finance_currency_numeric_code (
                    _unique         INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
                    code            INTEGER   NOT NULL  UNIQUE,
                    symbol          TEXT      NOT NULL  UNIQUE,
                    created_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
                    updated_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP
                );
        
                CREATE TRIGGER IF NOT EXISTS update_finance_currency_numeric_code_updated_at
                AFTER UPDATE ON finance_currency_numeric_code
                FOR EACH ROW
                BEGIN
                    UPDATE finance_currency_numeric_code
                    SET updated_at = CURRENT_TIMESTAMP
                    WHERE _unique = OLD._unique;
                END;
        
                -- https://en.wikipedia.org/wiki/ISO_4217
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('AUD', 036);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('CAD', 124);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('CNY', 156);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('HKD', 344);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('JPY', 392);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('KRW', 410);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('SGD', 702);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('CHF', 756);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('THB', 764);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('GBP', 826);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('USD', 840);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('TWD', 901);
                INSERT OR IGNORE INTO finance_currency_numeric_code (symbol, code) VALUES ('EUR', 978);
            "
        }
    }

    impl super::NumericCode {
        pub fn select_all() -> Result<Vec<Self>> {
            let sql = "SELECT code, symbol FROM finance_currency_numeric_code";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let items = statement.query_map([], |row| {
                Ok(Self {
                    code: row.get(0)?,
                    symbol: row.get(1)?,
                })
            })?;

            let mut result = Vec::new();
            for item in items {
                result.push(item?);
            }

            Ok(result)
        }

        pub fn select_one_by_code(code: i64) -> Result<Option<Self>> {
            let sql =
                "SELECT code, symbol FROM finance_currency_numeric_code WHERE code = ?1 LIMIT 1";
            let connection = connection()?;
            let mut statement = connection.prepare(sql)?;

            let item = statement
                .query_row([code], |row| {
                    Ok(Self {
                        code: row.get(0)?,
                        symbol: row.get(1)?,
                    })
                })
                .optional()?;

            Ok(item)
        }
    }
}
