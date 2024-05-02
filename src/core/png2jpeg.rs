use crate::{utils::magick, Config, Dimensions, PathAccessor};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), magick_rust::MagickError>
where
    T: PathAccessor + Dimensions,
{
    magick::optimize(&Config {
        input_path: config.input_path(),
        output_path: config.output_path(),
        width: config.width(),
        height: config.height(),
    })
}

#[cfg(test)]
mod tests {

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
