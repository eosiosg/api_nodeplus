use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ContactUsRequest {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub message: String,
}
