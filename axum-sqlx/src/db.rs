// use sqlx::Connection;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub done: bool,
}

pub async fn add_todo(pool: &MySqlPool, description: String) -> anyhow::Result<u64> {
    // Insert the task, then obtain the ID of this row
    let todo_id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ? )
        "#,
        description
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(todo_id)
}

pub async fn complete_todo(pool: &MySqlPool, id: u64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &MySqlPool) -> anyhow::Result<Vec<Todo>> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|rec| Todo {
        id: rec.id,
        description: rec.description,
        done: rec.done != 0,
    })
    .collect();

    Ok(recs)
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::mysql::MySqlPool;
    use sqlx::Connection as OtherConnection;

    #[tokio::test]
    async fn test_mysql() {
        let pool = MySqlPool::connect("mysql://root:example@localhost/dbtest")
            .await
            .unwrap();
        list_todos(&pool).await;

        let mut conn = pool.acquire().await.unwrap();
        conn.ping().await.unwrap()
    }
}
