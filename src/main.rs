use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod controllers;
use controllers::user::{registration, login};
use std::env;
use sqlx::mysql::MySqlPoolOptions;

pub mod models;
pub mod utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // initialize tracing
    dotenv().expect(".env file not found");

    
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&env::var("DATABASE_URL")?).await?;
    // build our application with a route
    let app: Router = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users/registration", post(registration))
        .route("/users/login", post(login))

        .with_state(pool);


    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())

}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
