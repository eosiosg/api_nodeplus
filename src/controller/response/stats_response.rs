use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Stats {
    pub total_assets: i64,
    pub total_rewards: i64,
    pub total_running_time: i64,
    pub total_node_count: i64,
}
