table! {
    first (id) {
        id -> Uuid,
        time -> Timestamp,
        msg -> text,
    }
}

table! {
    second (id) {
        id -> Uuid,
        time -> Timestamp,
        first_id -> Uuid,
    }
}

joinable!(second -> first (first_id));

allow_tables_to_appear_in_same_query!(
    first,
    second,
);
