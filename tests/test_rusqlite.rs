#[cfg(test)]
mod rusqlite_tests {
    use rusqlite::Connection;
    #[test]
    pub fn test_rusqlite() {
        let rusqlite_connection = Connection::open("/tmp/file1").unwrap();
        rusqlite_connection.query_row("PRAGMA journal_mode=WAL;", [], |_| Ok(())).unwrap();
        rusqlite_connection.execute("CREATE TABLE IF NOT EXISTS test_table (column_a INTEGER)", []).unwrap();
        rusqlite_connection.execute("DELETE FROM test_table", []).unwrap();
        rusqlite_connection.execute("INSERT INTO test_table VALUES (1)", []).unwrap();
        for _ in 0..=20000 {
            let result: i32 = rusqlite_connection
                .query_row("SELECT column_a FROM test_table", [], |r| r.get(0))
                .unwrap();
            println!("Result {}", result);
        }
    }
}
