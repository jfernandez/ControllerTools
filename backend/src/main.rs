mod api;

use std::{fs::File, net::SocketAddr, sync::Arc};

use api::{Controller, API};
use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use tower_http::cors::{Any, CorsLayer};

const PORT: u16 = 33220;

#[tokio::main]
async fn main() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("/tmp/controller-tools.log").unwrap(),
        ),
    ]);

    let api = Arc::new(API {});
    let app = Router::new()
        .route("/controllers", get(controllers))
        .layer(
            CorsLayer::new()
                .allow_origin("https://steamloopback.host".parse::<HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods([Method::GET, Method::POST]),
        )
        .with_state(api);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn controllers(State(api): State<Arc<API>>) -> Result<Json<Vec<Controller>>, AppError> {
    // Spawn a blocking task to get the controllers. This is because `api.get_controllers()` is a blocking API
    let controllers = tokio::task::spawn_blocking(move || api.get_controllers()).await??;
    Ok(Json(controllers))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
