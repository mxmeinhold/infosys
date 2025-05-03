use axum::{Router, routing::get, response::{Response, IntoResponse}, Json, http::StatusCode};
use tokio::sync::Mutex;
use serde::Serialize;

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

fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
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
    let app = init_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    
}

fn main(){ main1();}
