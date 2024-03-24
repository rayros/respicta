use axum::{
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Router,
};

async fn convert(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }
}

pub fn app(limit: Option<usize>) -> Router {
    let limit = limit.unwrap_or(10 * 1024 * 1024);
    Router::new()
        .route("/", post(convert))
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

        let app = Router::new().route("/", post(convert));
        let server = TestServer::new(app).unwrap();
        let image_bytes = include_bytes!("../README.md");
        let image_part = Part::bytes(image_bytes.as_slice())
            .file_name("README.md")
            .mime_type("text/markdown");

        let multipart_form = MultipartForm::new().add_part("file", image_part);
        let response = server.post("/").multipart(multipart_form).await;

        assert_eq!(response.text(), "");
    }
}
