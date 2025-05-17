use sqlx::SqliteConnection;

pub async fn create_tutor_table(conn: &mut SqliteConnection) {
    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS tutor(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(200) NOT NULL,
        age INTEGER NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    );
    ";

    sqlx::query(create_table_sql).execute(conn).await.unwrap();

    println!("Tutor table is created !")
}

pub async fn create_first_tutor(conn: &mut SqliteConnection) {
    let create_table_sql = "
        INSERT INTO tutor (name, age)
        VALUES ('First Tutor', 30);
    ";

    sqlx::query(create_table_sql).execute(conn).await.unwrap();

    println!("First tutor is created !")
}
