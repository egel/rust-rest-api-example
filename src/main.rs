use axum::{Router, routing::get};

mod controllers;


#[tokio::main]
async fn main() {
    // vars
    let address: String = "0.0.0.0:8000".to_string();

    let app = Router::new().route("/", get(controllers::root));

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server started on {}", &address);

    axum::serve(listener, app).await.unwrap();
}
