// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Integer,
        group_name -> Varchar,
    }
}

diesel::table! {
    history_log (history_id) {
        history_id -> Integer,
        time -> Datetime,
        group_id -> Integer,
        package_id -> Integer,
    }
}

diesel::table! {
    packages (package_id) {
        package_id -> Bigint,
        url -> Varchar,
        package_name -> Varchar,
        metric_one -> Tinyint,
        metric_two -> Tinyint,
        metric_three -> Tinyint,
        metric_four -> Tinyint,
        metric_five -> Tinyint,
        metric_six -> Tinyint,
        metric_seven -> Tinyint,
        total_score -> Tinyint,
    }
}

diesel::table! {
    registry (registry_id) {
        registry_id -> Integer,
        request_id -> Integer,
        package_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    requests (request_id) {
        request_id -> Integer,
        request_type -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    history_log,
    packages,
    registry,
    requests,
    users,
);
