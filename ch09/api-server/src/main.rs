use api_server::db::init_dbpool;
use api_server::router::create_router;
use api_server::tracing::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();

    let dbpool = init_dbpool().await.expect("couldn't initialize DB pool");

    let router = create_router(dbpool).await;

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .expect("couldn't bind address");

    axum::serve(listener, router)
        .await
        .expect("unable to start server");
}
