pub mod db;
pub mod http_utils;
pub mod routes;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use routes::{
    expense_list::get_expense_lists,
    user::{create_user, get_users},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() {
    // Add tracing_subscriber to enable logging to stdout. Default log level
    // is DEBUG, and it can be changed with RUST_LOG env variable. Tracing can
    // be enabled on source-level too, for example:
    // RUST_LOG=info,tower_http=debug
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Read environment variables from .env file
    if let Err(error) = dotenv() {
        tracing::warn!("Failed to load variables from .env file: \"{}\". Make sure that environment variables are set some other way.", error)
    }

    let db_connection_pool = db::client::establish_connection_pool();

    // build our application with a single route
    let app = Router::new()
        .route("/expense-lists", get(get_expense_lists))
        .route("/users", get(get_users).post(create_user))
        .with_state(db_connection_pool)
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Starting server on {address}");
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
