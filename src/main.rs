use std::sync::Arc;

use db::Db;
use settings::Settings;
use tokio_cron_scheduler::{Job, JobScheduler};
mod db;
mod err;
mod settings;
mod speedy;

async fn run_test() -> tokio::task::JoinHandle<()> {
    let settings = Arc::new(Settings::new());
    let cron = settings.cron.clone();
    let db = Arc::new(Db::create(&settings.db_url).await.expect("db init"));
    db.migrate().await;
    let sched = JobScheduler::new().await.unwrap();
    sched
        .add(
            Job::new_async(cron.as_str(), move |_uuid, _l| {
                let job_db = Arc::clone(&db);
                let cron_settings = settings.clone();
                Box::pin(async { speedy::run(cron_settings.into(), job_db).await })
            })
            .unwrap(),
        )
        .await
        .expect("cron add failed");
    sched.start().await.unwrap()
}

#[tokio::main]
async fn main() {
    let scheduler = run_test();
    let longtimer = tokio::time::sleep(std::time::Duration::from_secs(999999));
    let _ = tokio::join!(scheduler, longtimer);
    ()
}
