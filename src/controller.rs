use rocket::fairing::AdHoc;

pub mod request;
mod response;
mod contact_us_controller;
mod stats_controller;

use super::service;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn route() -> AdHoc{
    AdHoc::on_ignite("mount route",|rocket| async {
        rocket
            .attach(service::db_service::Db::fairing())
            .mount("/", routes![stats_controller::stats, contact_us_controller::contact_us])
    })
}

pub fn cors() -> AdHoc{
    AdHoc::on_ignite("cors",|rocket| async{
        let cors = CorsOptions::default()
            .allowed_origins(AllowedOrigins::all())
            .allowed_methods(
                vec![Method::Get, Method::Post, Method::Patch,Method::Options, Method::Put, Method::Head]
                    .into_iter()
                    .map(From::from)
                    .collect(),
            )
            .allow_credentials(true);
        rocket
            .attach(cors.to_cors().unwrap())
    })
}