// @generated automatically by Diesel CLI.

diesel::table! {
    person (_unique) {
        _unique -> Integer,
        nickname -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
