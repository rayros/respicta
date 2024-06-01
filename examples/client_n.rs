async fn send_request() {
    let multipart = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(
            include_bytes!("../tests/files/convert_test1.JPG").to_vec(),
        )
        .file_name("convert_test1.JPG"),
    );
    reqwest::Client::new()
        .post("http://localhost:3000/")
        .query(&[("extension", "webp"), ("width", "100"), ("height", "100")])
        .multipart(multipart)
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    // send request to server n times
    for _ in 0..100 {
        send_request().await;
    }
}
