use axum::{Router, routing::get};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::api::{ping, todo_create, todo_delete, todo_list, todo_read, todo_update};

pub async fn create_router(dbpool: sqlx::Pool<sqlx::Sqlite>) -> axum::Router {
    Router::new()
        .route("/alive", get(|| async { "ok" }))
        .route("/ready", get(ping))
        .nest(
            "/v1",
            Router::new()
                .route("/todos", get(todo_list).post(todo_create))
                .route(
                    "/todos/{id}",
                    get(todo_read).put(todo_update).delete(todo_delete),
                ),
        )
        .with_state(dbpool)
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
