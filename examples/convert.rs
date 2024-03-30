use respicta::convert;

fn main() {
    convert(&respicta::Config {
        input_path: &"images/logo.jpeg".into(),
        output_path: &"images/logo_small.jpeg".into(),
        width: Some(200),
        height: Some(200),
    })
    .unwrap();
}
