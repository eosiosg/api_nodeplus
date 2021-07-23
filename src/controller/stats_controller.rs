use rocket::serde::json::Json;
use crate::controller::response::stats_response::Stats;
use crate::controller::response::base::Response;
use crate::service::db_service::Db;
use crate::service::db_service;

#[get("/stats")]
pub async fn stats(db:Db) -> Json<Response<Stats>> {
    match db_service::get_stats(db).await{
        Ok(r) => {
            Json(Response::<Stats> {
                code: 0,
                message: String::from("success"),
                data: Stats {
                    total_assets: r.total_assets,
                    total_rewards: r.total_rewards,
                    total_running_time: r.total_running_time,
                    total_node_count: r.total_node_count,
                },
            })
        },
        Err(e)=>{
            Json(Response::<Stats> {
                code: -2000,
                message: e,
                data: Stats {
                    total_assets: 0,
                    total_rewards: 0,
                    total_running_time: 0,
                    total_node_count: 0,
                },
            })
        }
    }


}
