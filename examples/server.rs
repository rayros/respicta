use respicta::server::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = "0.0.0.0:3000";
    let app = app(None);
    let listener = TcpListener::bind(address).await;
    match listener {
        Ok(listener) => {
            println!("Server started at http://{address}");
            axum::serve(listener, app).await
        }
        Err(error) => Err(error),
    }
}
