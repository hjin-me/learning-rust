use axum::extract::State;
use axum::Json;

use crate::db;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Connection;

#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: sqlx::MySqlPool,
}

pub async fn ping(State(s): State<AppState>) -> impl IntoResponse {
    let mut c = s.mysql_pool.acquire().await.unwrap();
    c.ping().await.unwrap();
    "pong"
}

pub async fn list_all(State(s): State<AppState>) -> impl IntoResponse {
    let todos = db::list_todos(&s.mysql_pool).await.unwrap();
    json!(todos).to_string()
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AddTodo {
    description: String,
}

pub async fn add_todo(State(s): State<AppState>, Json(b): Json<AddTodo>) -> impl IntoResponse {
    let todo_id = db::add_todo(&s.mysql_pool, b.description).await.unwrap();
    json!(todo_id).to_string()
}
