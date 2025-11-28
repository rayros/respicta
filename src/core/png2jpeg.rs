use crate::{utils::magick, Dimensions, PathAccessor, Quality};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Magick error: {0}")]
    Magick(magick::Error),
}

impl From<magick::Error> for Error {
    fn from(err: magick::Error) -> Self {
        Error::Magick(err)
    }
}

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), Error>
where
    T: PathAccessor + Dimensions + Quality,
{
    magick::optimize(config, None)?;

    Ok(())
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
