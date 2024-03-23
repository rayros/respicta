use axum::{
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Router,
};

async fn accept_form(mut multipart: Multipart) {
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

pub async fn run(address: Option<String>, limit: Option<usize>) -> std::io::Result<()> {
    let address = address.unwrap_or_else(|| "0.0.0.0:3000".to_string());
    let limit = limit.unwrap_or(10 * 1024 * 1024);
    let app = Router::new()
        .route("/", post(accept_form))
        .layer(DefaultBodyLimit::max(limit));
    let listener = tokio::net::TcpListener::bind(address.clone()).await;
    match listener {
        Ok(listener) => {
            println!("Server started at http://{address}");
            axum::serve(listener, app).await
        }
        Err(error) => Err(error),
    }
}
