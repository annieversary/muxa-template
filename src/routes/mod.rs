use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use maud::html;
use muxa::{config::Config, errors::*, router::RouterExtension, sessions::UserSession};
use sqlx::SqlitePool;

use crate::html::HtmlContextBuilder;

pub fn router(pool: SqlitePool, config: Config) -> Router {
    Router::new()
        .typed_get(home::home)
        .layer(default_layers! {
            builder: HtmlContextBuilder,
            pool: pool,
            config: config.clone(),
            extensions: [],
        })
        .static_dirs(&config, &["css", "js"])
        .fallback(handler_404)
}

mod home;

muxa::routes! {
    "/" => Home {}
}

/// fallback handler for not-defined routes
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "page not found")
}
