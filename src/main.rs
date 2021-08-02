#[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_sync_db_pools;
// #[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;
// extern crate dotenv;


mod controller;
mod service;

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .attach(controller::route())
        .attach(controller::cors())
        .attach(service::cron())
        .launch()
        .await
    {
        println!("server didn't launch");
        drop(e);
    };
}
