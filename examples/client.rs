use std::io::Write;

#[tokio::main]
async fn main() {
    let multipart = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(
            include_bytes!("../tests/files/convert_test1.JPG").to_vec(),
        )
        .file_name("convert_test1.JPG"),
    );
    let response = reqwest::Client::new()
        .post("http://localhost:3000/")
        .query(&[("extension", "jpeg"), ("width", "100"), ("height", "100")])
        .multipart(multipart)
        .send()
        .await
        .unwrap();

    // save response to file
    let mut file = std::fs::File::create("response.jpeg").unwrap();
    file.write_all(&response.bytes().await.unwrap()).unwrap();
}
