extern crate dotenv;

use dotenv::dotenv;
use std::env;

use axum::{
    routing::{ get, post },
    http::StatusCode,
    Json, Router,
};
use serde::{ Deserialize, Serialize };

#[tokio::main]
fn main() {
    // Load environment
    dotenv().ok();

    let db_instance = env::var("DB_INSTANCE");
    let db_user = env::var("DB_USER");
    println!("db_instance: {:?}, db_user: {:?}", db_instance, db_user);

    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
