// @generated automatically by Diesel CLI.

diesel::table! {
    packages (id) {
        id -> Bigint,
        view_time -> Datetime,
        url -> Varchar,
        user_agent -> Varchar,
        device_type -> Tinyint,
    }
}
