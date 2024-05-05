use crate::app_error::AppError;
use crate::{convert, Config};
use axum::extract::Query;
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::Response,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use tokio::fs::{read, write};

#[derive(Deserialize, Serialize)]
pub struct Params {
    extension: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

async fn convert_method(
    params: Query<Params>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let output_extension = params.extension.clone().unwrap_or(String::from("webp"));
    let tempdir = tempdir()?;
    let field = multipart.next_field().await?.unwrap();
    let file_name = field.file_name().unwrap();
    let input_path = tempdir.path().join(file_name);
    let output_path = input_path.with_extension(output_extension);
    let data = field.bytes().await?;
    write(&input_path, &data).await?;
    convert(&Config::new(
        &input_path,
        &output_path,
        params.width,
        params.height,
    ))?;
    let file_content = read(&output_path).await?;
    let body = Body::from(file_content);
    let response = Response::builder().status(StatusCode::OK).body(body)?;
    Ok(response)
}

pub fn app(limit: Option<usize>) -> Router {
    let limit = limit.unwrap_or(10 * 1024 * 1024);
    Router::new()
        .route("/", post(convert_method))
        .layer(DefaultBodyLimit::max(limit))
}

mod tests {

    #[tokio::test]
    async fn test_app() {
        use super::*;
        use axum::http::StatusCode;
        use axum_test::TestServer;

        let app = app(None);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_convert() {
        use super::*;
        use axum_test::multipart::MultipartForm;
        use axum_test::{multipart::Part, TestServer};

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();
        let image_bytes = include_bytes!("../tests/files/issue-159.png");
        let image_part = Part::bytes(image_bytes.as_slice()).file_name("issue-159.png");

        let multipart_form = MultipartForm::new().add_part("file", image_part);
        let response = server.post("/").multipart(multipart_form).await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_convert_no_multipart() {
        use super::*;
        use axum_test::TestServer;

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();

        let response = server.post("/").await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.text(),
            "Invalid `boundary` for `multipart/form-data` request"
        );
    }

    #[tokio::test]
    async fn test_convert_unknown_extension() {
        use super::*;
        use axum_test::multipart::MultipartForm;
        use axum_test::{multipart::Part, TestServer};

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();
        let image_bytes = include_bytes!("../tests/files/issue-159.png");
        let image_part =
            Part::bytes(image_bytes.as_slice()).file_name("issue-159.someunknownextension");

        let multipart_form = MultipartForm::new().add_part("file", image_part);
        let response = server.post("/").multipart(multipart_form).await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response.text(),
            "Unsupported conversion: someunknownextension -> webp"
        );
    }

    #[tokio::test]
    async fn test_query_params() {
        use super::*;
        use axum_test::multipart::MultipartForm;
        use axum_test::{multipart::Part, TestServer};

        let app = Router::new().route("/", post(convert_method));
        let server = TestServer::new(app).unwrap();
        let image_bytes = include_bytes!("../tests/files/issue-159.png");
        let image_part = Part::bytes(image_bytes.as_slice()).file_name("issue-159.png");

        let multipart_form = MultipartForm::new().add_part("file", image_part);
        let response = server
            .post("/")
            .add_query_params(Params {
                extension: Some("jpeg".to_string()),
                width: Some(100),
                height: Some(100),
            })
            .multipart(multipart_form)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
