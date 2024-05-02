use respicta::convert;

fn main() {
    for _ in 0..100 {
        convert(&respicta::Config::new(
            "tests/files/convert_test1.JPG",
            "target/logo_small.webp",
            None,
            None,
        ))
        .unwrap();
    }
}
