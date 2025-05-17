use sqlx::SqliteConnection;

pub async fn create_course_table(conn: &mut SqliteConnection) {
    let create_table_sql = "
    CREATE TABLE IF NOT EXISTS course(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        tutor_id INTEGER NOT NULL,
        name VARCHAR(200) NOT NULL,
        description VARCHAR(200),
        posted_time TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT fk_tutor 
        FOREIGN KEY(tutor_id)
        REFERENCES tutor(id)
    );
    ";

    sqlx::query(create_table_sql).execute(conn).await.unwrap();

    println!("Course table is created !")
}

pub async fn create_first_course(conn: &mut SqliteConnection) {
    let create_table_sql = "
        INSERT INTO course (tutor_id, name, description)
        VALUES (1, 'First Course', 'This is the first course');
    ";

    sqlx::query(create_table_sql).execute(conn).await.unwrap();

    println!("The first course is created !")
}
