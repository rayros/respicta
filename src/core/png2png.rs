use thiserror::Error;

use crate::utils::magick;
use crate::{utils, Config};

use crate::{Dimensions, PathAccessor};

use super::PathIO;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Magick({0})")]
    Magick(magick::Error),
    #[error("Oxipng({0})")]
    Oxipng(oxipng::PngError),
    #[error("Io({0})")]
    Io(std::io::Error),
}

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    let output_path = config.output_path();
    let step1_output_path = &output_path.with_extension("step1.png");
    let magick_config = Config::new(
        config.input_path(),
        step1_output_path,
        config.width(),
        config.height(),
    );
    utils::magick::optimize(&magick_config, None).map_err(Error::Magick)?;
    let oxipng_config = PathIO::new(step1_output_path, config.output_path());
    utils::oxipng::optimize(&oxipng_config).map_err(Error::Oxipng)?;
    std::fs::remove_file(step1_output_path).map_err(Error::Io)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn png2png_test1() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2png_test1.png",
            "target/png2png_test1.png",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    fn png2png_test2() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2png_test2.png",
            "target/png2png_test2.png",
            Some(100),
            Some(100),
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Magick(MagickError(\"unable to open image 'tests/files/png2png_notexisting_test1.png':"]
    fn png2png_panic() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2png_notexisting_test1.png",
            "target/png2png_test1.png",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
