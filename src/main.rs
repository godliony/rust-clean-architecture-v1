use axum::{http::Method, Router};
use rust_clean_architecture_v1::{database, setting::Setting};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, Cors, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new().unwrap();
    info!("setting has been loaded");

    let db_pool = database::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0,0,0,0], setting.server.port as u16));

    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server running on port {}", setting.server.port);
    axum::serve(listener, app).await.unwrap();
}
