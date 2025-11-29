use crate::{
    utils::webp::{self},
    Dimensions, PathAccessor, Quality,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("PNG to WebP conversion error: {0}")]
    WebP(webp::Error),
}

impl From<webp::Error> for Error {
    fn from(err: webp::Error) -> Self {
        Error::WebP(err)
    }
}

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions + Quality,
{
    webp::optimize(config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2webp_test1.png",
            "target/png2webp_test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
