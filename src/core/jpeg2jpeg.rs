use magick_rust::FilterType;

use crate::{utils::magick, Dimensions, PathAccessor, Quality};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), magick::Error>
where
    T: PathAccessor + Dimensions + Quality,
{
    magick::optimize(config, Some(FilterType::Lanczos))
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
    #[should_panic = "MagickError(\"unable to open image 'tests/files/jpeg2jpeg_notexisting_test1.jpg':"]
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
