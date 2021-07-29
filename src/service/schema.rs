table! {
    chain_stat (id) {
        id -> Nullable<Integer>,
        chain -> Text,
        k -> Double,
        c -> Double,
        past_total_assets -> Double,
        past_total_rewards -> Double,
        past_total_running_time -> Double,
        total_node_count -> Double,
        updated_at -> Timestamp,
        daily_reward -> Double,
        daily_running_time -> Double,
        market_price_usd -> Double,
        market_price_time -> Timestamp,
    }
}

table! {
    contact_us (id) {
        id -> Nullable<Integer>,
        name -> Text,
        phone -> Text,
        email -> Text,
        message -> Text,
        status -> Nullable<Integer>,
        created_at -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    chain_stat,
    contact_us,
);
