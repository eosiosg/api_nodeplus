pub mod db_service;
pub mod schema;

use rocket::fairing::AdHoc;
use cronjob::CronJob;
use rocket::tokio::runtime::Runtime;

pub fn cron() -> AdHoc {
    AdHoc::on_liftoff("cron service", |_rocket| Box::pin(async move {
        fn on_cron(_name: &str) {
            Runtime::new().unwrap().block_on(
                db_service::notify_contact_us()
            );
        }

        let mut cron = CronJob::new("cron service", on_cron);
        cron.seconds("0");
        // cron.minutes("0");
        // Start the cronjob.
        CronJob::start_job_threaded(cron);
    }))
}