mod controllers;
mod envs;
mod migrations;
mod models;
mod router;

#[tokio::main]
async fn main() {
    envs::load_dotenv();

    let migrations = tokio::task::spawn_blocking(migrations::start_migrations)
        .await
        .expect("migrations task failed");
    migrations.expect("database migrations failed");

    let app = router::create_router();
    let address: String = format!("{}:{}", envs::host(), envs::port()).to_string();
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server started on {}", &address);

    axum::serve(listener, app).await.unwrap();
}
