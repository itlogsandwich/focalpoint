use tracing::{info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use axum::Router;
use axum::routing::get;
use crate::error::Error;

mod error;

async fn health_check() -> impl axum::response::IntoResponse
{
    println!("--> {:<12} - Health Check -" , "HANDLER");

    axum::response::Html("<h1> The server is definitely running </h1>")
}

fn router() -> Router
{
    Router::new()
        .route("/", get(health_check)) 
}

type MainError<T> = Result<T, Error>;
#[tokio::main]
async fn main() -> MainError<()>
{
    //This is boilerplate code for sanity checks
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "server=debug,axum=info,tower_http=debug".into()
        }))
        .init();
    
    let app = router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969")
        .await?;
    
    info!("Server runnning on http://localhost:6969");
    axum::serve(listener,app).await?;

    info!("Oh, this is gonna be fucking hell.");

    Ok(())
}
