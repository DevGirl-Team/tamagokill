use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Redirect,
    routing::{get, post},
    Json, Router, 
};
use serde::{Deserialize, Serialize};

use utoipa::{
    OpenApi,
    ToSchema
};
use utoipauto::utoipauto;
use utoipa_swagger_ui::SwaggerUi;

#[utoipauto(
    paths = "./src"
)]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tamagokill API", 
        description = "API for the Tamagokill project.",
        version = "0.1.0"
    )
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { Redirect::permanent("/swagger-ui") }))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Creates a new user
///
/// Creates a new user and returns the user.
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "The user created", body = User)
    )
)]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
#[derive(ToSchema)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
#[derive(ToSchema)]
struct User {
    id: u64,
    username: String,
}