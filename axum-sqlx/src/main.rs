use crate::api::AppState;
use axum::routing::{get, post};
use axum::Router;
mod api;
mod db;

#[tokio::main]
async fn main() {
    let mysql_dsn = std::env::var("DATABASE_URL").expect("环境变量里面没有 DATABASE_URL");
    let pool = sqlx::MySqlPool::connect(&mysql_dsn)
        .await
        .expect("初始化数据库连接池失败");

    let app_state = AppState { mysql_pool: pool };
    // // build our application with a single route
    let app = Router::new()
        .route("/liveness", get(api::ping))
        .route("/list_all", get(api::list_all))
        .route("/add_todo", post(api::add_todo))
        .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
