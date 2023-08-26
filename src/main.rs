use axum::{
    routing::{get, post, delete},
    http::StatusCode,
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

mod controllers;
use controllers::{user::{registration, login}, bank::{create_bank, list_all_banks,get_bank,update_bank,delete_bank,create_bank_entry, list_bank_entry}};
use std::env;
use sqlx::mysql::MySqlPoolOptions;

pub mod models;
pub mod utils;
pub mod middlewares;
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
        
        .route("/banks/create", post(create_bank))
        .route("/banks/:id", post(update_bank))
        .route("/banks/:id", get(get_bank))
        .route("/banks/:id", delete(delete_bank))
        .route("/banks", get(list_all_banks))

        .route("/bank-entrys/:bank_id/create", post(create_bank_entry))
        .route("/bank-entrys/:bank_id/:id", post(update_bank))
        //.route("/bank-entry/:bank_id/:id", get(get_bank))
        //.route("/bank-entry/:bank_id/:id", delete(delete_bank))
        .route("/bank-entrys/:bank_id", get(list_bank_entry))


        .layer(CorsLayer::permissive())
        .with_state(pool);

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let port = port.parse::<u16>().unwrap();
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
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
