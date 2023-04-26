// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Int4,
        group_name -> Varchar,
    }
}

diesel::table! {
    history_log (history_id) {
        history_id -> Int4,
        time_stamp -> Nullable<Timestamp>,
        group_id -> Int4,
        package_id -> Int4,
        request_id -> Int4,
    }
}

diesel::table! {
    packages (package_id) {
        package_id -> Int4,
        link -> Varchar,
        package_name -> Varchar,
        metric_one -> Int4,
        metric_two -> Int4,
        metric_three -> Int4,
        metric_four -> Int4,
        metric_five -> Int4,
        metric_six -> Int4,
        metric_seven -> Int4,
        total_score -> Int4,
    }
}

diesel::table! {
    registry (registry_id) {
        registry_id -> Int4,
        group_id -> Int4,
        package_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    requests (request_id) {
        request_id -> Int4,
        request_type -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
    }
}

diesel::joinable!(history_log -> groups (group_id));
diesel::joinable!(history_log -> packages (package_id));
diesel::joinable!(history_log -> requests (request_id));
diesel::joinable!(registry -> groups (group_id));
diesel::joinable!(registry -> packages (package_id));
diesel::joinable!(registry -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    history_log,
    packages,
    registry,
    requests,
    users,
);
