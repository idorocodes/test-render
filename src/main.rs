use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct Response {
    pub message: String,
    pub status_code: u16,
}

impl Response {
    pub fn init() -> Self {
        Response {
            message: String::from(""),
            status_code: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Request {
    name: String,
}

async fn home_route() -> impl IntoResponse {
    let mut response = Response::init();

    response.message = String::from("WOrking fine, render is redering");
    response.status_code = StatusCode::OK.as_u16();

    Json(response)
}

async fn name_path(Json(request): Json<Request>) -> impl IntoResponse {
    let mut response = Response::init();

    if request.name.is_empty() {
        response.message = String::from("Name not supplied");
        response.status_code = StatusCode::BAD_REQUEST.as_u16();
        return Json(response);
    };

    response.message = format!("Welcome {}", request.name);
    response.status_code = StatusCode::OK.as_u16();

    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_route))
        .route("/name", post(name_path));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on port 30000");

    axum::serve(listener, app).await.unwrap();
}
