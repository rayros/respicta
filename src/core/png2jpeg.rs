use crate::{utils::magick, Dimensions, InputOutput};

pub fn convert<T>(config: &T) -> std::result::Result<(), magick_rust::MagickError>
where
    T: InputOutput + Dimensions,
{
    magick::optimize(&magick::Config {
        input_path: config.input_path(),
        output_path: config.output_path(),
        width: config.width(),
        height: config.height(),
    })
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2jpeg() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/png2jpeg_test1.png".into(),
            output_path: &"target/png2jpeg_test1.jpeg".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
