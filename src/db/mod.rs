use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?; 

    Ok(pool)
}
