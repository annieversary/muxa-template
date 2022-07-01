use axum::{
    extract::Extension, handler::Handler, http::StatusCode, middleware, response::IntoResponse,
    routing::get_service, Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use maud::html;
use muxa::{
    config::Config,
    errors::*,
    html::*,
    sessions::{session_middleware, DbSessionStore, UserSession},
};
use sqlx::SqlitePool;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::html::{HtmlContextBuilder, InternalContext};

pub fn router(pool: SqlitePool, config: Config) -> Router {
    let mut css_path = config.get_static_path().clone();
    css_path.push("css");

    let mut js_path = config.get_static_path().clone();
    js_path.push("js");

    Router::new()
        .typed_get(home::home)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(pool.clone()))
                .layer(Extension(config))
                .layer(Extension(
                    DbSessionStore::new(pool).with_same_site(muxa::cookies::SameSite::Lax),
                ))
                .layer(middleware::from_fn(session_middleware))
                .layer(middleware::from_fn(
                    HtmlMiddleware::<_, InternalContext, NamedRoute>::html_context_middleware,
                )),
        )
        .nest(
            "/css",
            get_service(ServeDir::new(css_path)).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .nest(
            "/js",
            get_service(ServeDir::new(js_path)).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .fallback(handler_404.into_service())
}

mod home;

muxa::routes! {
    "/" => Home {}
}

/// fallback handler for not-defined routes
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "page not found")
}
