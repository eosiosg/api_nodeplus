use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub(crate) code: i64,
    pub(crate) message: String,
    pub(crate) data: T,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmptyData {}
