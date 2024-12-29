// @generated automatically by Diesel CLI.

diesel::table! {
    finance_currency_numeric_code (_unique) {
        _unique -> Integer,
        code -> Integer,
        symbol -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    finance_currency_transaction (_unique) {
        _unique -> Integer,
        owner -> Integer,
        amount -> Integer,
        numeric_code -> Integer,
        remarks -> Nullable<Text>,
        is_publish -> Bool,
        occurrence_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    finance_currency_transaction_category (_unique) {
        _unique -> Integer,
        owner -> Integer,
        descp -> Nullable<Text>,
        is_publish -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    person (_unique) {
        _unique -> Integer,
        nickname -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(finance_currency_transaction -> person (owner));
diesel::joinable!(finance_currency_transaction_category -> person (owner));

diesel::allow_tables_to_appear_in_same_query!(
    finance_currency_numeric_code,
    finance_currency_transaction,
    finance_currency_transaction_category,
    person,
);
