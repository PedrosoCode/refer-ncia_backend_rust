use sqlx::PgPool;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://postgres:Inspiron1@localhost:5432/gest";
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}
