use crate::{utils::magick, Dimensions, PathAccessor};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), magick::Error>
where
    T: PathAccessor + Dimensions,
{
    magick::optimize(config, None)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2jpeg() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2jpeg_test1.png",
            "target/png2jpeg_test1.jpeg",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
