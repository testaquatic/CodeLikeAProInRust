use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, sqlite::SqliteQueryResult, Error, SqlitePool};

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub async fn list(dbpool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as("SELECT * FROM todos").fetch_all(&dbpool).await
    }

    pub async fn read(dbpool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(&dbpool)
            .await
    }

    pub async fn create(dbpool: SqlitePool, new_todo: CreateTodo) -> Result<Todo, Error> {
        query_as("INSERT INTO todos (body) VALUES (?) RETURNING *")
            .bind(new_todo.body())
            .fetch_one(&dbpool)
            .await
    }

    pub async fn update(
        dbpool: SqlitePool,
        id: i64,
        updated_todo: UpdateTodo,
    ) -> Result<Todo, Error> {
        query_as(
            r#"UPDATE todos SET body = ?, completed = ?, updated_at = datetime('now') WHERE id = ? RETURNING *"#,
        ).bind(updated_todo.body())
        .bind(updated_todo.completed())
        .bind(id)
        .fetch_one(&dbpool)
        .await
    }

    pub async fn delete(dbpool: SqlitePool, id: i64) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&dbpool)
            .await
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateTodo {
    body: String,
}

impl CreateTodo {
    fn body(&self) -> &str {
        &self.body
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
}

impl UpdateTodo {
    fn body(&self) -> &str {
        &self.body
    }

    fn completed(&self) -> bool {
        self.completed
    }
}
