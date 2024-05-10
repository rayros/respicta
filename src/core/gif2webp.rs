use thiserror::Error;

use crate::{
    utils::{gifsicle, webp},
    Config, Dimensions, PathAccessor,
};

use super::PathIO;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Gifsicle({0})")]
    Gifsicle(gifsicle::Error),
    #[error("Io({0})")]
    Io(std::io::Error),
}

/// # Errors
///
/// Returns an error if the gifsicle command fails or if the input file does not exist.
///
pub fn convert<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    let output_path = config.output_path();
    let step1_output_path = &output_path.with_extension("step1");
    let gifsicle_config = Config::new(
        config.input_path(),
        step1_output_path,
        config.width(),
        config.height(),
    );

    gifsicle::optimize(&gifsicle_config).map_err(Error::Gifsicle)?;
    let webp_config = PathIO::new(step1_output_path, config.output_path());
    webp::optimize_gif(&webp_config).map_err(Error::Io)?;
    std::fs::remove_file(step1_output_path).map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn gif2webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/gif2webp_test1.gif",
            "target/gif2webp_test1.webp",
            Some(100),
            Some(100),
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Gifsicle(Exit(1))"]
    fn gif2webp_panic() {
        use super::*;

        convert(&Config::new(
            "tests/files/gif2webp_notexisting_test1.gif",
            "target/gif2webp_test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
