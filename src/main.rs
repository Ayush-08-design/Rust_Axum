use axum::{routing::get , Router , extract::{Path} , response::Json};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {

    // Axum's Router maps HTTP methods and paths to handler functions:
    let app = Router::new()
    .route("/" , get(root))
    .route("/json/{value}", get(json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await.unwrap();
    println!("server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

    async fn root() -> &'static str{
        "Heyyyyy World"
    }

    async fn json(Path(value): Path<u32>) -> Json<Value> {
        Json(json!({ "data": value }))
    }
}
