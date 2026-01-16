extern crate serde;
extern crate serde_json;
extern crate jsonapi;
extern crate axum;
extern crate utoipa_axum;
extern crate utoipa;

extern crate sqlx;


mod users;

use utoipa_axum::routes;
use sqlx::sqlite::SqlitePoolOptions;

use axum::{
    Json, routing::get
};
use utoipa_axum::router::OpenApiRouter;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new().connect("sqlite:./db.sqlite").await.expect("Result");
    let (router, openapi) = OpenApiRouter::new()
        .routes(routes!(users::handlers::get_user, users::handlers::patch_user, users::handlers::delete_user))
        .routes(routes!(users::handlers::get_users, users::handlers::post_user))
        .with_state(pool)
        .split_for_parts();

    let router = router
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/docs", get(|| async {
            Json(openapi)
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}