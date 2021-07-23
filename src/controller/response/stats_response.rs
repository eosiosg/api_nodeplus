use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Stats {
    pub total_assets: i32,
    pub total_rewards: i32,
    pub total_running_time: i32,
    pub total_node_count: i32,
}
