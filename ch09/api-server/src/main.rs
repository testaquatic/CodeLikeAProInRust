use std::str::FromStr;

use api_server::router::create_router;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();

    let dbpool = init_dbpool().await.expect("couldn't initialize DB pool");

    let router = create_router(dbpool).await;

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("couldn't bind to address");

    axum::serve(listener, router)
        .await
        .expect("unable to start server");
}

fn init_tracing() {
    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or("sqlx=debug,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let db_connection_str = std::env::var("DATABASE_URL").unwrap_or("sqlite:db.sqlite".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("can't connect to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("database migration failed");

    Ok(dbpool)
}
