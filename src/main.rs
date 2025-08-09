use dotenvy::dotenv;
use leloko_rust::models::{self, post::{self, Post}};
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};

use axum::{
    http::StatusCode, routing::get, Extension, Json, Router
};

use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABSE_URL must be set.");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database");

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts))
        .layer(Extension(pool));

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    info!("Server is running on port http://127.0.0.1:5000");
    axum::serve(listener,app).await.unwrap(); 

    Ok(())
}

// handler for http get
async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}