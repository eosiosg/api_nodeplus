use rocket::fairing::AdHoc;

pub mod request;
mod response;
mod contact_us_controller;
mod stats_controller;

use super::service;

pub fn route() -> AdHoc{
    AdHoc::on_ignite("mount route",|rocket| async {
        rocket
            .attach(service::db_service::Db::fairing())
            .mount("/", routes![stats_controller::stats, contact_us_controller::contact_us])
    })
}