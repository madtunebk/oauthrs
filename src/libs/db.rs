use sqlx::PgPool;

pub async fn connect(database_url: &str) -> PgPool {
    PgPool::connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_postgres_connection() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = connect(&database_url).await;

        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await
            .expect("Query failed");

        assert_eq!(row.0, 1);
        println!("PostgreSQL ping: ok");

        let version: (String,) = sqlx::query_as("SELECT version()")
            .fetch_one(&pool)
            .await
            .expect("Version query failed");

        println!("PostgreSQL version: {}", version.0);
    }
}
