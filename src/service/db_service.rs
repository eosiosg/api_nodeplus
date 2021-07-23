use rocket_sync_db_pools::{diesel, database};
use rocket::serde::{Serialize, Deserialize};

use super::schema::*;
use crate::controller::request::contact_us_request::ContactUsRequest;
use self::diesel::prelude::*;
use chrono;
use dotenv::dotenv;
use std::env;
use crate::service::schema::stats::columns::total_running_time;

#[database("diesel")]
pub struct Db(diesel::SqliteConnection);


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "contact_us"]
pub struct ContactUs {
    id: Option<i32>,
    name: String,
    email: String,
    message: String,
    status: i32,
}

#[derive(Queryable, Insertable)]
#[table_name = "stats"]
pub struct Stats {
    id: Option<i32>,
    pub total_assets: i32,
    pub total_rewards: i32,
    pub total_running_time: i32,
    pub total_node_count : i32,
    updated_at: chrono::NaiveDateTime,
}

pub async fn save_contact_us(db: Db, request: ContactUsRequest) -> Result<(), String> {
    let new_contact_us = ContactUs {
        id: Option::None,
        name: request.name,
        email: request.email,
        message: request.message,
        status: 0,
    };
    if let Err(e) = db.run(move |conn| {
        diesel::insert_into(contact_us::table)
            .values(new_contact_us)
            .execute(conn)
    }).await {
        println!("[save_contact_us] error: {}", e);
        return Err(format!("{}",e));
    }
    Ok(())
}

pub async fn get_stats(db: Db) -> Result<Stats,String> {
    match db.run(move |conn| {
        stats::table
            .filter(stats::id.eq(1))
            .first(conn)
    }).await {
        Ok(r) => Ok(r),
        Err(e) => {
            println!("[get_stats] error: {}", e);
            Err(format!("{}",e))
        }
    }
}

pub fn update_stats(){
    println!("work work");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // let some_stats = stats::table.filter(stats::id.eq(1))
    //     .limit(1)
    //     .load::<Stats>(&connection)
    //     .expect("error");

    if let Err(e) = diesel::update(stats::table.find(1))
        .set(total_running_time.eq(total_running_time+3752))
        .execute(&connection){
        println!("{}",e);
    }
}