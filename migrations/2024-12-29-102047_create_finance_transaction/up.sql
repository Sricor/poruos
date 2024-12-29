CREATE TABLE finance_currency_transaction (
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

CREATE TRIGGER update_finance_currency_transaction_updated_at
AFTER UPDATE ON finance_currency_transaction
FOR EACH ROW
BEGIN
    UPDATE finance_currency_transaction
    SET updated_at = CURRENT_TIMESTAMP
    WHERE _unique = OLD._unique;
END;
