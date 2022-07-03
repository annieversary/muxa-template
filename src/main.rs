use sqlx::sqlite::SqlitePoolOptions;

use muxa::{config::Config, errors, tracing::setup_tracing};
use muxa_template::routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to load .env");

    let log_path = std::env::var("LOG_PATH")
        .expect("failed to get LOG_PATH")
        .into();
    let _guard1 = setup_tracing(log_path);
    errors::setup_panic_hook();

    let db_connection_str = std::env::var("DATABASE_URL").expect("failed to get db url");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("failed to connect to db");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("couldn't run migrations");

    muxa::css::generate_css_from_inventory("static/css/zephyr.css")
        .await
        .unwrap();

    let config = Config::from_env();

    let app = routes::router(pool, config);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|a| a.parse().ok())
        .unwrap_or(3000);

    #[cfg(debug_assertions)]
    println!("listening on http://localhost:{port}");

    axum::Server::bind(&([0, 0, 0, 0], port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
