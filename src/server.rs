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

pub async fn run() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", post(accept_form))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    match listener {
        Ok(listener) => {
            println!("Server started at http://localhost:3000");
            axum::serve(listener, app).await
        }
        Err(error) => Err(error),
    }
}
