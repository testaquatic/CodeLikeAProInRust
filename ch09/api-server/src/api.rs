use crate::error::Error;
use crate::todo::{CreateTodo, Todo, UpdateTodo};
use axum::extract::Path;
use axum::{extract::State, Json};
use sqlx::{Connection, SqlitePool};

pub async fn ping(State(dbpool): State<SqlitePool>) -> Result<String, Error> {
    let mut conn = dbpool.acquire().await?;
    conn.ping()
        .await
        .map(|_| "ok".to_string())
        .map_err(Into::into)
}

pub async fn todo_list(State(dbpool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, Error> {
    Todo::list(dbpool).await.map(Json::from).map_err(Into::into)
}

pub async fn todo_read(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    Todo::read(dbpool, id)
        .await
        .map(Json::from)
        .map_err(Into::into)
}

pub async fn todo_create(
    State(dbpool): State<SqlitePool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::create(dbpool, new_todo)
        .await
        .map(Json::from)
        .map_err(Into::into)
}

pub async fn todo_update(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::update(dbpool, id, updated_todo)
        .await
        .map(Json::from)
        .map_err(Into::into)
}

pub async fn todo_delete(
    State(dbpool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), Error> {
    Todo::delete(dbpool, id).await?;

    Ok(())
}
