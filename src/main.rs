use crate::state::AppState;
mod models;
mod state;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {

    // Axum's Router maps HTTP methods and paths to handler functions:
    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await.unwrap();
    println!("server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

}
