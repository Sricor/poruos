CREATE TABLE person (
    _unique      INTEGER   NOT NULL  UNIQUE  PRIMARY KEY AUTOINCREMENT,
    nickname     TEXT      NOT NULL,
    password     TEXT      NOT NULL,
    created_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP,
    updated_at   DATETIME  NOT NULL  DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_person_updated_at
AFTER UPDATE ON person
FOR EACH ROW
BEGIN
    UPDATE person
    SET updated_at = CURRENT_TIMESTAMP
    WHERE _unique = OLD._unique;
END;

INSERT INTO sqlite_sequence (name, seq) VALUES ('person', 9999);
