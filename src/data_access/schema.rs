// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        description -> Text,
        completed -> Bool,
        session_id -> Uuid,
    }
}
