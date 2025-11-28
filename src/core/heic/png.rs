use std::{
    fs::{create_dir_all, write},
    io::Cursor,
};

use crate::{
    utils::{
        fit,
        webp::{rgba_to_webp, LibWebPError, RGBAImage},
    },
    Dimensions, PathAccessor, Quality,
};
use libheif_rs::{ColorSpace, HeifContext, HeifError, LibHeif, RgbChroma};
use oxipng::{optimize_from_memory, Options};
use png::{BitDepth, ColorType, Encoder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid input path")]
    WrongInputPath,
    #[error("HEIC error: {0}")]
    Heif(HeifError),
    #[error("No interleaved plane found")]
    NoInterleavedPlane,
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("PNG encoding error: {0}")]
    Png(png::EncodingError),
    #[error("Optimization error: {0}")]
    Optimization(oxipng::PngError),
}

impl From<HeifError> for Error {
    fn from(err: HeifError) -> Self {
        Error::Heif(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Error::Png(err)
    }
}

impl From<oxipng::PngError> for Error {
    fn from(err: oxipng::PngError) -> Self {
        Error::Optimization(err)
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
    let (target_width, target_height) = fit(
        handle.width(),
        handle.height(),
        config.width().unwrap_or(handle.width()),
        config.height().unwrap_or(handle.height()),
    );
    let image = lib_heif
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?
        .scale(target_width, target_height, None)?;

    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or(Error::NoInterleavedPlane)?;

    let mut png_bytes: Vec<u8> = Vec::new();
    let cursor = Cursor::new(&mut png_bytes);
    let mut encoder = Encoder::new(
        cursor,
        interleaved_plane.width as u32,
        interleaved_plane.height as u32,
    );
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&interleaved_plane.data)?;
    writer.finish()?;

    let options = &Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    // 2. Perform the optimization from memory
    let optimized_bytes = optimize_from_memory(
        &png_bytes, // Input unoptimized PNG data
        &options,   // Optimization settings
    )?;

    if let Some(parent) = config.output_path().parent() {
        create_dir_all(parent)?;
    }

    write(config.output_path(), optimized_bytes)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ConfigBuilder;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_heic_to_png_conversion() {
        let input_path = PathBuf::from("tests/files/sample.heic");
        let output_path = PathBuf::from("target/test_heic_to_png_conversion.png");

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
