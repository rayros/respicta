use std::io::Write;

#[tokio::main]
async fn main() {
    let multipart = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(include_bytes!("../tests/files/issue-159.png").to_vec())
            .file_name("issue-159.png"),
    );
    let response = reqwest::Client::new()
        .post("http://localhost:3000/")
        .header("extension", "jpeg")
        .multipart(multipart)
        .send()
        .await
        .unwrap();

    // save response to file
    let mut file = std::fs::File::create("response.jpeg").unwrap();
    file.write_all(&response.bytes().await.unwrap()).unwrap();
}
