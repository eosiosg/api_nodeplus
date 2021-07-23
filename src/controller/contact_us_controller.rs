use rocket::serde::json::Json;
use crate::controller::response::base::{EmptyData, Response};
use crate::controller::request::contact_us_request::ContactUsRequest;
use crate::service::db_service::Db;
use crate::service::db_service;

#[post("/contact_us", data = "<contact_us_request>")]
pub async fn contact_us(db:Db, contact_us_request: Json<ContactUsRequest>) -> Json<Response<EmptyData>> {
    println!("{:?}", contact_us_request);
    if let Err(e) = db_service::save_contact_us(db,contact_us_request.into_inner())
        .await{
        return Json(Response::<EmptyData> {
            code: -1000,
            message: e,
            data: EmptyData {},
        });
    }

    Json(Response::<EmptyData> {
        code: 0,
        message: String::from("success"),
        data: EmptyData {},
    })
}

