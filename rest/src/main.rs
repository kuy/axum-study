extern crate dotenv;

use axum::prelude::*;
use dotenv::dotenv;
use std::net::SocketAddr;

mod handlers;
mod views;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app = route("/", any(handlers::not_found)).nest("/beans", handlers::beans::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
