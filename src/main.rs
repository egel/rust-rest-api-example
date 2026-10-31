//! Binary entry point. All application modules live in the library crate and
//! are exposed via `rust_rest_api_example::…` — see `src/lib.rs`.

use tracing::info;

use rust_rest_api_example::{db, envs, logging, migrations, router};

#[tokio::main]
async fn main() {
    envs::load_dotenv();
    logging::init_logging();

    info!(environment = envs::environment(), "starting server");

    let migrations = tokio::task::spawn_blocking(migrations::start_migrations)
        .await
        .expect("migrations task failed");
    migrations.expect("database migrations failed");
    info!("database migrations applied");

    let pool = db::connect(envs::database_url())
        .await
        .expect("database pool failed");
    info!("database pool connected");

    let app = router::create_router(pool);
    let address: String = format!("{}:{}", envs::host(), envs::port()).to_string();
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    info!(address, "server started");

    axum::serve(listener, app).await.unwrap();
}
