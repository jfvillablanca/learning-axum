use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn hello() -> Html<String> {
    Html("<h1>Hello World</h1>".into())
}
