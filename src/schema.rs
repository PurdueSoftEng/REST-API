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
        time_stamp -> Timestamp,
        user_id -> Int4,
        package_id -> Int4,
        request_id -> Int4,
    }
}

diesel::table! {
    packages (package_id) {
        package_id -> Int4,
        id -> Varchar,
        url -> Nullable<Varchar>,
        version -> Varchar,
        package_name -> Varchar,
        jsprogram -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        ramp_up -> Nullable<Float4>,
        bus_factor -> Nullable<Float4>,
        resp_maintain -> Nullable<Float4>,
        license -> Nullable<Float4>,
        correct -> Nullable<Float4>,
        good_practice -> Nullable<Float4>,
        pull_request -> Nullable<Float4>,
        net_score -> Nullable<Float4>,
    }
}

diesel::table! {
    registry (registry_id) {
        registry_id -> Int4,
        group_id -> Int4,
        package_id -> Int4,
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
        isadmin -> Bool,
        token -> Varchar,
    }
}

diesel::joinable!(history_log -> packages (package_id));
diesel::joinable!(history_log -> requests (request_id));
diesel::joinable!(history_log -> users (user_id));
diesel::joinable!(registry -> groups (group_id));
diesel::joinable!(registry -> packages (package_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    history_log,
    packages,
    registry,
    requests,
    users,
);
