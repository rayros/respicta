use std::{
    fs::{create_dir_all, write},
    io::Cursor,
};

use crate::{utils::fit, Dimensions, PathAccessor};
use libheif_rs::{ColorSpace, HeifContext, HeifError, LibHeif, RgbChroma};
use oxipng::{optimize_from_memory, Options};
use png::{BitDepth, ColorType, Encoder};
use resize::Pixel::RGBA8;
use resize::Type::Lanczos3;
use rgb::FromSlice;
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
    #[error("Resizing error: {0}")]
    Resize(resize::Error),
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

impl From<resize::Error> for Error {
    fn from(err: resize::Error) -> Self {
        Error::Resize(err)
    }
}

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), Error>
where
    T: PathAccessor + Dimensions,
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
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;

    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or(Error::NoInterleavedPlane)?;

    let src = interleaved_plane.data.as_rgba();
    let dst_size = target_width * target_height;
    let mut dst = vec![rgb::RGBA8::new(0, 0, 0, 0); dst_size as usize];

    let mut resizer = resize::new(
        interleaved_plane.width as usize,
        interleaved_plane.height as usize,
        target_width as usize,
        target_height as usize,
        RGBA8,
        Lanczos3,
    )?;

    resizer.resize(src, &mut dst)?;

    let after_resize = dst
        .iter()
        .map(|px| [px.r, px.g, px.b, px.a])
        .flatten()
        .collect::<Vec<u8>>();

    let mut png_bytes: Vec<u8> = Vec::new();
    let cursor = Cursor::new(&mut png_bytes);
    let mut encoder = Encoder::new(cursor, target_width as u32, target_height as u32);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&after_resize)?;
    writer.finish()?;

    let options = &Options {
        strip: oxipng::StripChunks::Safe,
        ..Options::default()
    };

    let optimized_bytes = optimize_from_memory(&png_bytes, &options)?;

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

        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
