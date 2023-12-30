use axum::{routing::get, Router};
use std::net::SocketAddr;

use handler_lib::functions::get_response::get_response;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1", get(get_response));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
