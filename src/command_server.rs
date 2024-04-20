use std::path::PathBuf;

use crate::app_error::AppError;
use crate::{convert, Config};
use axum::{body::Body, http::StatusCode, response::Response, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Command {
    pub input_path: String,
    pub output_path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

async fn convert_method(Json(payload): Json<Command>) -> Result<Response, AppError> {
    convert(&Config {
        input_path: &PathBuf::from(&payload.input_path),
        output_path: &PathBuf::from(&payload.output_path),
        width: payload.width,
        height: payload.height,
    })?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?;
    Ok(response)
}

pub fn app() -> Router {
    Router::new().route("/", post(convert_method))
}
