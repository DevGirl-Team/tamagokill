use sea_orm::{ConnectionTrait, Database, Statement, DbErr};
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenvy::dotenv().ok();
    let db_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL in the .env file or at program startup.");
    let url = db_url;

    // TODO : Create table if doesn't exists
    let db = Database::connect(&url)
        .await
        .expect("Database connection failed");

    Ok(())
}
