use crate::utils::webp;

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

pub fn convert(config: &Config) -> std::result::Result<(), libwebp_sys::WebPEncodingError> {
    webp::optimize(&webp::Config {
        input_path: config.input_path,
        output_path: config.output_path,
        width: config.width,
        height: config.height,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn jpeg2webp() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/jpeg2webp_test1.jpeg",
            output_path: "target/jpeg2webp_test1.webp",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
