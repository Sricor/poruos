CREATE TABLE finance_currency_numeric_code (
    _unique         INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
    code            INTEGER   NOT NULL  UNIQUE,
    symbol          TEXT      NOT NULL  UNIQUE,
    created_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_finance_currency_numeric_code_updated_at
AFTER UPDATE ON finance_currency_numeric_code
FOR EACH ROW
BEGIN
    UPDATE finance_currency_numeric_code
    SET updated_at = CURRENT_TIMESTAMP
    WHERE _unique = OLD._unique;
END;

-- https://en.wikipedia.org/wiki/ISO_4217
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('AUD', 036);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('CAD', 124);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('CNY', 156);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('HKD', 344);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('JPY', 392);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('KRW', 410);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('SGD', 702);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('CHF', 756);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('THB', 764);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('GBP', 826);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('USD', 840);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('TWD', 901);
INSERT INTO finance_currency_numeric_code (symbol, code) VALUES ('EUR', 978);
