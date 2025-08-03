#[cfg(test)]
mod sqlx_tests {
    use sqlx::Connection;
    use sqlx::Row;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::sqlite::SqliteConnection;

    #[tokio::test]
    pub async fn test_sqlx() {
        let options = SqliteConnectOptions::new()
            .filename("/tmp/file2")
            .synchronous(sqlx::sqlite::SqliteSynchronous::Off)
            .create_if_missing(true);
        let mut sqlx_connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::query("PRAGMA journal_mode=WAL;")
            .fetch_one(&mut sqlx_connection)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS test_table (column_a INTEGER)")
            .execute(&mut sqlx_connection)
            .await
            .unwrap();
        sqlx::query("DELETE FROM test_table").execute(&mut sqlx_connection).await.unwrap();
        sqlx::query("INSERT INTO test_table VALUES (1)").execute(&mut sqlx_connection).await.unwrap();

        for _ in 0..=20000 {
            let result: i32 = sqlx::query("SELECT column_a FROM test_table")
                .fetch_one(&mut sqlx_connection)
                .await
                .unwrap()
                .get(0);
            println!("Result {}", result);
        }
    }
}
