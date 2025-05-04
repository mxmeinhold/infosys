use axum::{Router, routing::get, response::{Response, IntoResponse}, Json, http::StatusCode};
use tokio::sync::Mutex;
use serde::Serialize;
use std::sync::Arc;
use cron_tab::CronError;
use cron_tab::Cron;
use chrono::{FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;

//GLOBAL VARIABLES
static mut CURRENT_MODE : u8 = 0;
static mut MODES : Vec<u8> = vec![];
//END GLOBAL


// here we show a type that implements Serialize + Send
#[derive(Serialize)]
struct Message {
    message: String
}

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn tour_cron(cr : Cron<Local>) -> &'static str {
    //remove cron from current mode, 
    
    unsafe { CURRENT_MODE = 0 };

    return stringify!(MODES[TOUR]);
}

// #[axum::debug_handler]
fn init_router(cr: Cron<Local>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tour", get(tour_cron(cr)))
}


impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}


#[tokio::main]
async fn main1() {
    let local_tz = Local::from_offset(&FixedOffset::west_opt(5).unwrap());
    let mut cron = AsyncCron::new(local_tz);

    

    let app = init_router(cron);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    
}

fn main(){ main1();}
