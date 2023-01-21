mod api;
mod settings;
mod ws;

use std::{fs::File, net::SocketAddr, sync::Arc};

use api::Controller;

use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use log::info;
use settings::Settings;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

use tower_http::cors::{Any, CorsLayer};

use crate::settings::SettingsService;

const PORT: u16 = 33220;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub struct AppState {
    settings_service: SettingsService,
}

#[tokio::main]
async fn main() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LOG_LEVEL,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LOG_LEVEL,
            Config::default(),
            File::create("/tmp/controller-tools.log").unwrap(),
        ),
    ]);

    let settings_location = match tokio::fs::metadata("/home/deck/homebrew/settings").await {
        Ok(_) => "/home/deck/homebrew/settings/controller-tools.json",
        Err(_) => "/tmp/controller-tools.json",
    };

    let settings = SettingsService::new(&settings_location).await.unwrap();
    let app_state = Arc::new(AppState {
        settings_service: settings,
    });

    let app = Router::new()
        .route("/controllers", get(controllers_json))
        .route("/settings", get(get_settings).post(post_settings))
        .route("/ws", get(ws::ws_handler))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin("https://steamloopback.host".parse::<HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods([Method::GET, Method::POST]),
        );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_settings(State(state): State<Arc<AppState>>) -> Result<Json<Settings>, AppError> {
    let settings = state.settings_service.get_settings().await;
    Ok(Json(settings))
}

async fn post_settings(
    State(state): State<Arc<AppState>>,
    Json(settings): Json<Settings>,
) -> Result<Json<Settings>, AppError> {
    let settings = state.settings_service.set_settings(settings).await?;
    Ok(Json(settings))
}

async fn controllers_json() -> Result<Json<Vec<Controller>>, AppError> {
    // Spawn a tokio blocking task because `get_controllers()` is a blocking API
    let controllers = tokio::task::spawn_blocking(api::controllers).await??;
    Ok(Json(controllers))
}

// Make our own error that wraps `anyhow::Error`
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
