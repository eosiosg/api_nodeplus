pub mod db_service;
pub mod schema;

use rocket::fairing::AdHoc;
use cronjob::CronJob;
use rocket::tokio::runtime::Runtime;

pub fn cron() -> AdHoc {
    AdHoc::on_liftoff("cron service", |_rocket| Box::pin(async move {
        fn on_cron_notify_contact_us(_name: &str) {
            Runtime::new().unwrap().block_on(
                db_service::notify_contact_us()
            );
        }

        let mut cron_notify_contact_us = CronJob::new("cron service notify_contact_us", on_cron_notify_contact_us);
        cron_notify_contact_us.seconds("0");
        CronJob::start_job_threaded(cron_notify_contact_us);


        fn on_cron_update_market_price(_name: &str){
            Runtime::new().unwrap().block_on(
                db_service::update_market_price()
            );
        }

        let mut cron_update_market_price = CronJob::new("cron service update market price",on_cron_update_market_price);
        cron_update_market_price.minutes("0");
        CronJob::start_job_threaded(cron_update_market_price);
    }))
}