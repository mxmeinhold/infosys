use infosys_display::{tour_mode, grab_from_db};

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use tokio::sync::Mutex;
use cron_tab::CronError;
use std::env;


//fn main() {
//    let app = Router::<()>::new()
//        .without_v07_checks()
//        .route("/colon", get(|| async {}))
//        .route("/*asterisk", get(|| async {}));
//}

// sec   min   hour   day of month   month   day of week   year
// *     *     *      *              *       *             *


#[tokio::main]
async fn main() -> Result<(), CronError>{
    let local_tz = Local::from_offset(&FixedOffset::west_opt(5).unwrap());
    let mut cron = AsyncCron::new(local_tz);
    let path = env::current_dir();
    println!("the Current directory is {}", path.expect("REASON").display());

    cron.add_fn("* * * * * * *", move || tour_mode()).await;
    //cron.add_fn("* 1 * * * * *", move || grab_from_db());
    
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
// hellow
async fn print_now() {
    println!("now: {}", Local::now().to_string());
}
