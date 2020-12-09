table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Bpchar,
        fingerprint -> Text,
        expires -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        login -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
