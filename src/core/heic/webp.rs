use std::fs::{create_dir_all, write};

use crate::{
    utils::webp::{rgba_to_webp, LibWebPError, RGBAImage},
    Dimensions, PathAccessor, Quality,
};
use libheif_rs::{ColorSpace, HeifContext, HeifError, LibHeif, RgbChroma};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid input path")]
    WrongInputPath,
    #[error("HEIC error: {0}")]
    Heif(HeifError),
    #[error("WebP error: {0}")]
    WebP(LibWebPError),
    #[error("No interleaved plane found")]
    NoInterleavedPlane,
    #[error("I/O error: {0}")]
    Io(std::io::Error),
}

impl From<HeifError> for Error {
    fn from(err: HeifError) -> Self {
        Error::Heif(err)
    }
}

impl From<LibWebPError> for Error {
    fn from(_err: LibWebPError) -> Self {
        // Map LibWebPError to a generic Error variant
        Error::WebP(_err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
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
    let lib_heif = LibHeif::new();
    let name = config.input_path().to_str().ok_or(Error::WrongInputPath)?;
    let ctx = HeifContext::read_from_file(name)?;
    let handle = ctx.primary_image_handle()?;
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;

    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or(Error::NoInterleavedPlane)?;

    let rgba_image = RGBAImage {
        data: interleaved_plane.data.as_ptr(),
        width: interleaved_plane.width,
        height: interleaved_plane.height,
    };

    let contents = rgba_to_webp(&rgba_image, config)?;

    if let Some(parent) = config.output_path().parent() {
        create_dir_all(parent)?;
    }

    write(config.output_path(), contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ConfigBuilder;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_heic_to_webp_conversion() {
        let input_path = PathBuf::from("tests/files/sample.heic");
        let output_path = PathBuf::from("target/test_heic_to_webp_conversion.webp");

        let config = ConfigBuilder::default()
            .input_path(input_path)
            .output_path(output_path.clone())
            .width(Some(200))
            .height(Some(200))
            .quality(Some(80))
            .build()
            .unwrap();

        let result = convert(&config);

        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
