mod entities;

use entities::{prelude::*, *};

use sea_orm::*;
use std::{env, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    /* DATABASE CONNECTION */
    dotenvy::dotenv().ok();
    let db_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL in the .env file or at program startup.");
    let db_name = env::var("DATABASE_NAME")
        .expect("No DATABASE_NAME in the .env file or at program startup.");

    // TODO : Create table if doesn't exists
    let db = Database::connect(&db_url)
        .await
        .expect("Database connection failed");

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", &db_name),
            ))
            .await?;

            let url = format!("{}/{}", &db_url, &db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", &db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", &db_name),
            ))
            .await?;

            let url = format!("{}/{}", &db_url, &db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    /* INIT API AXUM */
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .with_state(Arc::new(db));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
    State(state): State<Arc<DatabaseConnection>>,
) -> (StatusCode, Json<User>) {
    let dummy_user = user::ActiveModel {
        name: ActiveValue::Set(payload.name),
        ..Default::default()
    };
    let res = User::insert(dummy_user).exec(state.as_ref()).await.unwrap();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json({}))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    name: String,
}
