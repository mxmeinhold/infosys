use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use tokio::sync::Mutex;
use cron_tab::CronError;
//fn main() {
//    let app = Router::<()>::new()
//        .without_v07_checks()
//        .route("/colon", get(|| async {}))
//        .route("/*asterisk", get(|| async {}));
//}


#[tokio::main]
async fn main() -> Result<(), CronError>{
    let local_tz = Local::from_offset(&FixedOffset::west_opt(5).unwrap());
    let mut cron = AsyncCron::new(local_tz);
    

    cron.add_fn("* * * * * * *", print_now).await.unwrap();

    cron.start().await;

    let counter = Arc::new(Mutex::new(1));
    cron.add_fn("* * * * * * *", move || {
        let counter = counter.clone();
        async move {
            let mut counter = counter.lock().await;
            *counter += 1;
            let now = Local::now().to_string();
            println!("{} counter value: {}", now, counter);
        }
    })
    .await
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));

    // stop cron
    cron.stop().await;
    Ok(())
}

async fn print_now() {
    println!("now: {}", Local::now().to_string());
}
