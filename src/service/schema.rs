table! {
    contact_us (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        message -> Text,
        status -> Nullable<Integer>,
        created_at -> Nullable<Text>,
    }
}

table! {
    stats (id) {
        id -> Nullable<Integer>,
        total_assets -> Integer,
        total_rewards -> Integer,
        total_running_time -> Integer,
        total_node_count -> Integer,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    contact_us,
    stats,
);
