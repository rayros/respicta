use respicta::convert;

fn main() {
    convert(&respicta::Config::new(
        "images/logo.jpeg",
        "images/logo_small.jpeg",
        Some(200),
        Some(200),
    ))
    .unwrap();
}
