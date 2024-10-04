mod api;
mod controller;
mod settings;
mod ws;

use std::{fs::File, net::SocketAddr, sync::Arc};

use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use controller::Controller;
use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

use tower_http::cors::{Any, CorsLayer};

use crate::settings::SettingsService;

const PORT: u16 = 33220;

pub struct AppState {
    settings_service: SettingsService,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("Error: Expected 2 arguments, but got {}", args.len() - 1);
    }

    let settings_directory = args[1].clone();

    let settings_location = match tokio::fs::metadata(&settings_directory).await {
        Ok(_) => format!("{}/settings.json", &settings_directory),
        Err(_) => String::from("/tmp/controller-tools.json"),
    };
    let settings_service = SettingsService::new(&settings_location).await.unwrap();

    let level_filter = match settings_service.get_settings().await.debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };
    CombinedLogger::init(vec![
        TermLogger::new(
            level_filter,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            level_filter,
            Config::default(),
            File::create(args[2].clone()).unwrap(),
        ),
    ])
    .unwrap();

    let app_state = Arc::new(AppState { settings_service });

    let app = Router::new()
        .route("/controllers", get(controllers_json))
        .route("/ws", get(ws::ws_handler))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin("https://steamloopback.host".parse::<HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods([Method::GET, Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    info!("Logging level: {:?}", level_filter);
    info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
