use axum::Router;
use axum::routing::get;

async fn health_check() -> impl axum::response::IntoResponse
{
    tracing::info!("--> {:<12} - Health Check -" , "HANDLER");

    axum::response::Html("<h1> The server is definitely running </h1>")
}

pub fn app_router(state: crate::state::AppState) -> Router
{
    Router::new()
        .route("/", get(health_check))
        .with_state(state)
}
