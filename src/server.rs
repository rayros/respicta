use crate::{convert, Config};
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart},
    http::{header::HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use tempfile::tempdir;
use tokio::fs::{read, write};

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
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

async fn convert_method(
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let extension_header = headers.get("extension");
    let output_extension = extension_header
        .and_then(|ext| ext.to_str().ok())
        .unwrap_or("webp");
    let width: Option<u32> = headers
        .get("width")
        .and_then(|width| width.to_str().ok())
        .and_then(|width| width.parse().ok());
    let height: Option<u32> = headers
        .get("height")
        .and_then(|height| height.to_str().ok())
        .and_then(|height| height.parse().ok());
    let tempdir = tempdir()?;
    let field = multipart.next_field().await?.unwrap();
    let file_name = field.file_name().unwrap();
    let input_path = tempdir.path().join(file_name);
    let output_path = input_path.with_extension(output_extension);
    let data = field.bytes().await?;
    write(&input_path, &data).await?;
    convert(&Config {
        input_path: &input_path,
        output_path: &output_path,
        width,
        height,
    })?;
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

        // test start server
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
}
