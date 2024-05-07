use magick_rust::bindings::FilterType_LanczosFilter;

use crate::{utils::magick, Dimensions, PathAccessor};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), magick::Error>
where
    T: PathAccessor + Dimensions,
{
    magick::optimize(config, Some(FilterType_LanczosFilter))
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn jpeg2jpeg() {
        use super::*;

        convert(&Config::new(
            "tests/files/jpeg2jpeg_test1.jpg",
            "target/jpeg2jpeg_test1.jfif",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "MagickError(\"failed to read image\")"]
    fn jpeg2jpeg_panic() {
        use super::*;

        convert(&Config::new(
            "tests/files/jpeg2jpeg_notexisting_test1.jpg",
            "target/jpeg2jpeg_test1.jfif",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
