use axum::{
    Router,
    routing::{get, post},
    serve,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod speedtest;
mod viz;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/run-test", post(routes::run_speed_test))
        .route("/api/visualize", get(routes::get_visualization))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
