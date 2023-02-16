use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;

use handler_lib::functions::get_response::get_response;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1", get(get_response));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
