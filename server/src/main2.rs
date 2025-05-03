use infosys_display::{self, grab_from_db, tour_mode};

use axum::{Router, routing::get, routing::post};
use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use std::sync::Arc;
//use std::sync::Mutex;
use cron_tab::CronError;
use std::convert::Infallible;
use std::env;
use tokio::sync::Mutex;

//fn main() {
//    let app = Router::<()>::new()
//        .without_v07_checks()
//        .route("/colon", get(|| async {}))
//        .route("/*asterisk", get(|| async {}));
//}

// sec   min   hour   day of month   month   day of week   year
// *     *     *      *              *       *             *

#[tokio::main]
async fn main1() -> Result<(), CronError> {
    let local_tz = Local::from_offset(&FixedOffset::west_opt(5).unwrap());
    let mut cron = AsyncCron::new(local_tz);
    let path = env::current_dir();
    println!(
        "the Current directory is {}",
        path.expect("REASON").display()
    );
    let mut mode = Modes::Tour;
    let mut tour_id = 0;
    let mut db_id = 0;

    db_id = cron
        .add_fn("* 1 * * * * *", move || grab_from_db())
        .await
        .expect("usize");
    mode = Modes::Db;
    //cron.add_fn("* 1 * * * * *", move || grab_from_db());

    cron.start().await;

    let cron = Arc::new(Mutex::new(cron));
    /*
    || async {
                let cr = cron.lock().unwrap();
                cr.remove(db_mode.clone());
                match cr.add_fn("* 1 * * * * *", move || infosys_display::tour_mode()).await {
                    Ok(mode) => db_mode = mode,
                    Err(e) => eprintln!("Failed to add to cron {e}")
                };
            }
    let cr = cron.lock().unwrap();
                cr.remove(tour_mode);
                match cr.add_fn("* 1 * * * * *", move || grab_from_db()).await {
                    Ok(mode) => db_mode = mode,
                    Err(e) => eprintln!("Failed to add to cron: {e}"),
                };
    */
    // Routes ---------------------------------
    let app = Router::<()>::new()
        .without_v07_checks()
        .route("/api/insert", post(|| async {}))
        .route("/tour", get(|| async {}))
        //----------------------------------------------
        .route(
            "/db",
            get({
                //let cron = cron.clone();
                //|| function(cron)
            }),
        )
        //----------------------------------------------
        .route(
            "/stop",
            get({
                println!("/stop called");
                Ok::<(), Infallible>(cron.lock().await.stop().await)
            }),
        )
        //----------------------------------------------
        .route(
            "/start",
            get({
                println!("/start called");
                Ok::<(), Infallible>(cron.lock().await.start().await)
            }),
        )
        //----------------------------------------------
        .route(
            "/dbmode",
            get({
                println!("/dbmode called");
                Ok::<(), Infallible>(
                    match cron
                        .lock()
                        .await
                        .add_fn("* 1 * * * * *", async move || grab_from_db().await)
                        .await
                    {
                        Ok(m) => {
                            if mode == Modes::Tour {
                                cron.lock().await.remove(tour_id.clone());
                            }
                            db_id = m;
                            mode = Modes::Db
                        }
                        Err(e) => eprintln!("Failed to add to cron: {e}"),
                    },
                );
            }),
        )
        //---------------------------------------------
        .route(
            "/tourmode",
            get({
                println!("/tourmode called");
                Ok::<(), Infallible>(
                    match cron
                        .lock()
                        .await
                        .add_fn("* 1 * * * * *", async move || tour_mode().await)
                        .await
                    {
                        Ok(m) => {
                            if mode == Modes::Db {
                                cron.lock().await.remove(db_id.clone());
                            }
                            tour_id = m;
                            mode = Modes::Tour
                        }
                        Err(e) => eprintln!("Failed to add to cron: {e}"),
                    },
                );
            }),
        );
    //---------------------------------------------

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    // stop cron
    cron.lock().await.stop().await;
    Ok(())
}

//fn mode_switch(cr: Arc<Mutex<Cron>>, mode: Modes, new_mode: Modes, db_mode: usize){Ok(())}

#[derive(PartialEq)]
enum Modes {
    Tour,
    Db,
    Injected,
}

// hellow
async fn print_now() {
    println!("now: {}", Local::now().to_string());
}
