use axum::extract::{Json, Path, Query};
use axum::{Router, response::IntoResponse, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/health", get(health_check))
        .route("/user/{id}", get(get_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Do something else"
}
async fn get_foo() -> &'static str {
    "Getting Foo"
}
async fn foo_bar() -> &'static str {
    "Getting Foobar checking if things are working"
}

async fn get_user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({
        "status":"200",
        "message":"Received successfully",
        "data":{
            "userId":id
        }
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status":"ok",
        "message":"Server is running"
    }))
}
