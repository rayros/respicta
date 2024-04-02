use respicta::convert;

fn main() {
    for _ in 0..100 {
        convert(&respicta::Config {
            input_path: &"tests/files/convert_test1.JPG".into(),
            output_path: &"target/logo_small.webp".into(),
            width: None,
            height: None,
        })
        .unwrap();
    }
}
