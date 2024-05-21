use crate::app_error::AppError;
use crate::{convert, ConfigBuilder};
use axum::{body::Body, http::StatusCode, response::Response, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Command {
    pub input_path: String,
    pub output_path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub quality: Option<u32>,
}

async fn convert_method(Json(payload): Json<Command>) -> Result<Response, AppError> {
    convert(
        &ConfigBuilder::default()
            .input_path(&payload.input_path)
            .output_path(&payload.output_path)
            .width(payload.width)
            .height(payload.height)
            .quality(payload.quality)
            .build()
            .unwrap(),
    )?;
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

        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_convert() {
        use super::*;
        use axum_test::TestServer;

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();

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

    #[tokio::test]
    async fn test_convert_nested_dir() {
        use super::*;
        use axum_test::TestServer;

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/")
            .json(&serde_json::json!({
                "input_path": "tests/files/command_server_test1.jpg",
                "output_path": "target/command_server_nested/command_server_test1.webp",
                "width": 100,
                "height": 100,
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_convert_wrong_named_webp_jpg_to_jpg() {
        use super::*;
        use axum_test::TestServer;

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/")
            .json(&serde_json::json!({
                "input_path": "tests/files/webp/command_server_test2.jpg",
                "output_path": "target/jpg/command_server_test2.jpg",
                "width": 500,
            }))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
