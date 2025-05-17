use course::{create_course_table, create_first_course};
use tutor::{create_first_tutor, create_tutor_table};

pub mod course;
pub mod tutor;

pub async fn seed_tables(conn: &mut sqlx::SqliteConnection) {
    create_tutor_table(conn).await;
    create_first_tutor(conn).await;

    create_course_table(conn).await;
    create_first_course(conn).await;
}
