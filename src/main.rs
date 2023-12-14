use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/", get(|| async { Html("<h1>Hello World</h1>") }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    println!("=>> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
