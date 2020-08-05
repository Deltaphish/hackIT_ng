table! {
    completions (id) {
        id -> Int4,
        user -> Text,
        challenge_id -> Text,
        completion_time -> Nullable<Timestamp>,
    }
}

table! {
    records (id) {
        id -> Int4,
        name -> Text,
        challenge_id -> Text,
        toc -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    completions,
    records,
);
