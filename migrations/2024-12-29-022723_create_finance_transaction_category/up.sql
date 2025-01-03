CREATE TABLE finance_currency_transaction_category (
    _unique         INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
    owner           INTEGER   NOT NULL,
    remarks         TEXT,
    is_publish      BOOLEAN   NOT NULL  DEFAULT TRUE,
    created_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(owner) REFERENCES person(_unique)
);

CREATE TRIGGER update_finance_currency_transaction_category_at
AFTER UPDATE ON finance_currency_transaction_category
FOR EACH ROW
BEGIN
    UPDATE finance_currency_transaction_category
    SET updated_at = CURRENT_TIMESTAMP
    WHERE _unique = OLD._unique;
END;

