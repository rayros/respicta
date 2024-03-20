use crate::utils::magick;

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn convert(config: &Config) -> std::result::Result<(), magick_rust::MagickError> {
    magick::optimize(&magick::Config {
        input_path: config.input_path,
        output_path: config.output_path,
        width: config.width,
        height: config.height,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn jpeg2jpeg() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/jpeg2jpeg_test1.jpg",
            output_path: "target/jpeg2jpeg_test1.jfif",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn jpeg2jpeg_panic() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/jpeg2jpeg_notexisting_test1.jpg",
            output_path: "target/jpeg2jpeg_test1.jfif",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
