use std::path::PathBuf;

use crate::app_error::AppError;
use crate::{convert, Config};
use axum::{body::Body, http::StatusCode, response::Response, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

mod tests {
    #[tokio::test]
    async fn test_app() {
        use super::*;
        use axum::http::StatusCode;
        use axum_test::TestServer;

        let app = app();
        let server = TestServer::new(app).unwrap();

        // test start server
        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_convert() {
        use super::*;
        use axum_test::TestServer;

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();
        // TODO: Fix aspect ratio command_server_test1.webp
        let response = server
            .post("/")
            .json(&serde_json::json!({
                "input_path": "tests/files/command_server_test1.jpg",
                "output_path": "target/command_server_test1.webp",
                "width": 100,
                "height": 100,
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
