// use rocket::fairing::AdHoc;
//
pub mod db_service;
pub mod schema;
//
// use cronjob::CronJob;

// pub fn cron() -> AdHoc {
//     AdHoc::on_liftoff("cron service", |_rocket| Box::pin(async move {
//         fn on_cron(_name: &str) {
//             // println!("[{}]: ", name);
//             db_service::update_stats();
//         }
//
//         let mut cron = CronJob::new("cron service", on_cron);
//         cron.minutes("0");
//         // Start the cronjob.
//         CronJob::start_job_threaded(cron);
//     }))
// }