use rocket_sync_db_pools::{diesel, database};
use rocket::serde::{Serialize, Deserialize};

use super::schema::*;
use crate::controller::request::contact_us_request::ContactUsRequest;
use self::diesel::prelude::*;
use chrono;
use std::env;
use dotenv::dotenv;
use telegram_notifyrs;
use std::fmt::Write;


#[database("diesel")]
pub struct Db(diesel::SqliteConnection);


#[derive(Queryable, Insertable)]
#[table_name = "contact_us"]
pub struct ContactUs {
    id: Option<i32>,
    name: String,
    phone: String,
    email: String,
    message: String,
    status: i32,
    created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name = "chain_stat"]
pub struct ChainStat {
    id: Option<i32>,
    pub chain: String,
    pub k: f64,
    pub c: f64,
    pub past_total_assets: f64,
    pub past_total_rewards: f64,
    pub past_total_running_time: f64,
    pub total_node_count: f64,
    updated_at: chrono::NaiveDateTime,
    pub daily_reward: f64,
    pub daily_running_time: f64,
    pub market_price_usd: f64,
    pub market_price_time: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Stats {
    pub total_assets: i64,
    pub total_rewards: i64,
    pub total_running_time: i64,
    pub total_node_count: i64,
}


pub async fn save_contact_us(db: Db, request: ContactUsRequest) -> Result<(), String> {
    let new_contact_us = ContactUs {
        id: Option::None,
        name: request.name,
        phone: request.phone,
        email: request.email,
        message: request.message,
        status: 0,
        created_at: chrono::Utc::now().naive_utc()
    };
    if let Err(e) = db.run(move |conn| {
        diesel::insert_into(contact_us::table)
            .values(new_contact_us)
            .execute(conn)
    }).await {
        println!("[save_contact_us] error: {}", e);
        return Err(format!("{}", e));
    }
    Ok(())
}

pub async fn get_stats(db: Db) -> Result<Stats, String> {
    match db.run(move |conn| {
        chain_stat::table
            .load(conn)
    }).await {
        Ok::<Vec<ChainStat>, _>(chain_stats) => {
            let mut tmp_total_running_time = 0f64;
            let mut tmp_total_node_count = 0f64;
            let mut tmp_total_assets = 0f64;
            let mut tmp_total_rewards = 0f64;

            let now = chrono::Utc::now().naive_utc();

            for chain_stat in chain_stats {
                //total_node_count
                tmp_total_node_count += chain_stat.total_node_count * chain_stat.c;
                //total_running_time
                let mut chain_past_running_time = chain_stat.past_total_running_time;
                let passed_days = now.signed_duration_since(chain_stat.updated_at).num_days() as f64;
                chain_past_running_time += passed_days * chain_stat.daily_running_time * chain_stat.total_node_count * chain_stat.c;
                tmp_total_running_time += chain_past_running_time;
                //total_assets
                tmp_total_assets += chain_stat.past_total_assets * chain_stat.market_price_usd * chain_stat.k;
                //total_rewards
                let chain_total_rewards = chain_stat.past_total_rewards + chain_stat.daily_reward * chain_stat.total_node_count * chain_stat.c * passed_days;
                tmp_total_rewards += chain_total_rewards * chain_stat.market_price_usd * chain_stat.k;
            }

            Ok(Stats {
                total_running_time: tmp_total_running_time as i64,
                total_node_count: tmp_total_node_count as i64,
                total_assets: tmp_total_assets as i64,
                total_rewards: tmp_total_rewards as i64,
            })
        }
        Err(e) => {
            println!("[get_stats] error: {}", e);
            Err(format!("{}", e))
        }
    }
}



pub async fn notify_contact_us(){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    if let Ok(contact_us_records) = contact_us::table.filter(contact_us::status.eq(0))
        .load::<ContactUs>(&connection) {
        let mut msg = String::new();
        let mut ids = Vec::new();
        for r in contact_us_records{
            ids.push(r.id.unwrap());
            write!(&mut msg, "[name] {}\n[phone] {}\n[email] {}\n[message] {}\n[time] {}\n",r.name, r.phone,r.email, r.message,r.created_at).unwrap();
        }

        if ids.len() > 0 {
            let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
            let chat_id: i64 = env::var("TELEGRAM_CHAT_ID")
                .expect("Missing TELEGRAM_CHAT_ID environment variable")
                .parse()
                .expect("Error parsing TELEGRAM_CHAT_ID as i64");
            if telegram_notifyrs::send_message(msg, &token, chat_id).ok(){
                let _ = diesel::update(contact_us::table.filter(contact_us::id.eq_any(ids)))
                    .set(contact_us::status.eq(1))
                    .execute(&connection);
            }
        }
    }
}