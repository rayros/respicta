#[cfg(feature = "server-app-error")]
pub mod app_error;
#[cfg(feature = "command-server")]
pub mod command_server;
pub mod core;
pub mod extensions;
#[cfg(feature = "web-service")]
pub mod server;
pub mod utils;

use core::{gif2gif, gif2webp, heic, jpeg2jpeg, jpeg2webp, png2jpeg, png2png, png2webp, webp2webp};
use derive_builder::Builder;
use extensions::{GIF, HEIC, JFIF, JPEG, JPG, PNG, WEBP};
use std::path::{Path, PathBuf};
use thiserror::Error;
use utils::gifsicle;

pub trait PathAccessor {
    fn input_path(&self) -> &PathBuf;
    fn output_path(&self) -> &PathBuf;
}

pub trait Dimensions {
    fn width(&self) -> Option<u32>;
    fn height(&self) -> Option<u32>;
}

pub trait Quality {
    fn quality(&self) -> Option<u32>;
}

#[derive(Default, Builder, Debug)]
pub struct Config {
    #[builder(setter(into))]
    pub input_path: PathBuf,
    #[builder(setter(into))]
    pub output_path: PathBuf,
    #[builder(default)]
    pub width: Option<u32>,
    #[builder(default)]
    pub height: Option<u32>,
    #[builder(default)]
    pub quality: Option<u32>,
}

impl Config {
    pub fn new(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Config {
        Config {
            input_path: input_path.as_ref().to_path_buf(),
            output_path: output_path.as_ref().to_path_buf(),
            width,
            height,
            quality: None,
        }
    }
}

impl PathAccessor for Config {
    fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    fn output_path(&self) -> &PathBuf {
        &self.output_path
    }
}

impl Dimensions for Config {
    fn width(&self) -> Option<u32> {
        self.width
    }

    fn height(&self) -> Option<u32> {
        self.height
    }
}

impl Quality for Config {
    fn quality(&self) -> Option<u32> {
        self.quality
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Input file has no extension")]
    InputFileHasNoExtension,
    #[error("Output file has no extension")]
    OutputFileHasNoExtension,
    #[error("Unsupported conversion: {0} -> {1}")]
    UnsupportedConversion(String, String),
    #[error("Error converting png to png: {0}")]
    Png2Png(png2png::Error),
    #[error("Error converting png to jpg: {0}")]
    Png2Jpeg(png2jpeg::Error),
    #[error("Error converting png to webp: {0}")]
    Png2Webp(png2webp::Error),
    #[error("Error converting jpg to jpg: {0}")]
    Jpeg2Jpeg(jpeg2jpeg::Error),
    #[error("Error converting jpg to webp: {0}")]
    Jpeg2Webp(jpeg2webp::Error),
    #[error("Error converting gif to gif: {0}")]
    Gif2Gif(gifsicle::Error),
    #[error("Error converting gif to webp: {0}")]
    Gif2Webp(gif2webp::Error),
    #[error("Error converting webp to webp: {0}")]
    Webp2Webp(webp2webp::Error),
    #[error("Error converting heic to webp: {0}")]
    Heic2Webp(heic::webp::Error),
    #[error("Error converting heic to png: {0}")]
    Heic2Png(heic::png::Error),
    #[error("Error converting heic to jpeg: {0}")]
    Heic2Jpeg(heic::jpeg::Error),
}

impl From<gif2webp::Error> for Error {
    fn from(err: gif2webp::Error) -> Self {
        Error::Gif2Webp(err)
    }
}

impl From<gifsicle::Error> for Error {
    fn from(err: gifsicle::Error) -> Self {
        Error::Gif2Gif(err)
    }
}

impl From<png2webp::Error> for Error {
    fn from(err: png2webp::Error) -> Self {
        Error::Png2Webp(err)
    }
}

impl From<jpeg2jpeg::Error> for Error {
    fn from(err: jpeg2jpeg::Error) -> Self {
        Error::Jpeg2Jpeg(err)
    }
}

impl From<webp2webp::Error> for Error {
    fn from(err: webp2webp::Error) -> Self {
        Error::Webp2Webp(err)
    }
}

impl From<png2png::Error> for Error {
    fn from(err: png2png::Error) -> Self {
        Error::Png2Png(err)
    }
}

impl From<jpeg2webp::Error> for Error {
    fn from(err: jpeg2webp::Error) -> Self {
        Error::Jpeg2Webp(err)
    }
}

impl From<png2jpeg::Error> for Error {
    fn from(err: png2jpeg::Error) -> Self {
        Error::Png2Jpeg(err)
    }
}

impl From<heic::webp::Error> for Error {
    fn from(err: heic::webp::Error) -> Self {
        Error::Heic2Webp(err)
    }
}

impl From<heic::png::Error> for Error {
    fn from(err: heic::png::Error) -> Self {
        Error::Heic2Png(err)
    }
}

impl From<heic::jpeg::Error> for Error {
    fn from(err: heic::jpeg::Error) -> Self {
        Error::Heic2Jpeg(err)
    }
}

/// # Errors
///
/// Returns an error if:
/// * The input file path has no extension
/// * The output file path has no extension
/// * The conversion is not supported
/// * An error occurs during the conversion
///
pub fn convert(config: &Config) -> Result<(), Error> {
    match (
        config
            .input_path()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(str::to_lowercase),
        config
            .output_path()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(str::to_lowercase),
    ) {
        (Some(input_extension), Some(output_extension)) => {
            _convert(&input_extension, &output_extension, config)
        }
        (None, _) => Err(Error::InputFileHasNoExtension),
        (_, None) => Err(Error::OutputFileHasNoExtension),
    }
}

fn _convert(input_extension: &str, output_extension: &str, config: &Config) -> Result<(), Error> {
    let result: Result<(), Error> = match (input_extension, output_extension) {
        (GIF, GIF) => Ok(gif2gif::convert(config)?),
        (GIF, WEBP) => Ok(gif2webp::convert(config)?),
        (PNG, WEBP) => Ok(png2webp::convert(config)?),
        (WEBP, WEBP) => Ok(webp2webp::convert(config)?),
        (JPG | JPEG | JFIF, WEBP) => Ok(jpeg2webp::convert(config)?),
        (JPG | JPEG | JFIF, JPG | JPEG | JFIF) => Ok(jpeg2jpeg::convert(config)?),
        (PNG, PNG) => Ok(png2png::convert(config)?),
        (PNG, JPG | JPEG | JFIF) => Ok(png2jpeg::convert(config)?),
        (HEIC, WEBP) => Ok(heic::webp::convert(config)?),
        (HEIC, PNG) => Ok(heic::png::convert(config)?),
        (HEIC, JPG | JPEG | JFIF) => Ok(heic::jpeg::convert(config)?),
        (input_extension, output_extension) => Err(Error::UnsupportedConversion(
            input_extension.to_string(),
            output_extension.to_string(),
        )),
    };

    result
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn config() {
        use super::*;

        let config = Config::new("tests/files/test1.jpg", "target/test1.jpg", Some(100), None);

        assert_eq!(config.input_path(), &PathBuf::from("tests/files/test1.jpg"));
        assert_eq!(config.output_path(), &PathBuf::from("target/test1.jpg"));
        assert_eq!(config.width(), Some(100));
        assert_eq!(config.height(), None);
    }

    #[test]
    fn convert() -> Result<(), Error> {
        use super::*;

        convert(&Config::new(
            "tests/files/orientation_test.jpg",
            "target/convert_test1.webp",
            Some(100),
            None,
        ))?;

        convert(&Config::new(
            "tests/files/orientation_test.jpeg",
            "target/convert_test2.webp",
            Some(100),
            None,
        ))?;

        convert(&Config::new(
            "tests/files/convert_test1.png",
            "target/convert_test3.webp",
            None,
            None,
        ))?;

        convert(&Config::new(
            "tests/files/convert_test1.png",
            "target/convert_test4.webp",
            Some(10),
            None,
        ))?;

        convert(&Config::new(
            "tests/files/convert_test1.png",
            "target/convert_test5.webp",
            None,
            Some(10),
        ))?;

        convert(&Config::new(
            "tests/files/convert_test1.gif",
            "target/convert_test6.gif",
            Some(10),
            Some(10),
        ))?;

        Ok(())
    }

    #[test]
    fn convert_jfif_to_webp() -> Result<(), Error> {
        use super::*;

        convert(&Config::new(
            "tests/files/convert_test_jfif.jfif",
            "target/convert_test7.webp",
            Some(500),
            None,
        ))?;

        Ok(())
    }

    #[test]
    fn survive_extension_wrong_format_jpg_to_webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/convert_test2.jpg",
            "target/convert_test8.webp",
            Some(500),
            None,
        ))
        .unwrap();
    }

    #[test]
    fn extension_in_uppercase() {
        use super::*;

        convert(&Config::new(
            "tests/files/convert_test1.JPG",
            "target/convert_test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "UnsupportedConversion(\"jpg\", \"tiff\")"]
    fn convert_panic() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.jpg",
            "target/test1.tiff",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Jpeg2Webp(WebP(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })))"]
    fn convert_panic_jpg_to_webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.jpg",
            "target/test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Jpeg2Jpeg(Magick(Magick(MagickError(\"unable to open image 'tests/files/not_existing.jpg':"]
    fn convert_panic_jpg_to_jpg() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.jpg",
            "target/test1.jpg",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Png2Png(Magick(Magick(MagickError(\"unable to open image 'tests/files/not_existing.png':"]
    fn convert_panic_png_to_png() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.png",
            "target/test1.png",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Webp2Webp(WebP(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })))"]
    fn convert_panic_webp_to_webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.webp",
            "target/test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "InputFileHasNoExtension"]
    fn convert_panic_no_input_extension() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing",
            "target/test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "OutputFileHasNoExtension"]
    fn convert_panic_no_output_extension() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.jpg",
            "target/test1",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Gif2Gif(Exit(1))"]
    fn convert_panic_gif_to_gif() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.gif",
            "target/test1.gif",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Gif2Webp(Gifsicle(Exit(1)))"]
    fn convert_panic_gif_to_webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.gif",
            "target/test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Png2Webp(WebP(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })))"]
    fn convert_panic_png_to_webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.png",
            "target/test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Png2Jpeg(Magick(Magick(MagickError(\"unable to open image 'tests/files/not_existing.png':"]
    fn convert_panic_png_to_jpg() {
        use super::*;

        convert(&Config::new(
            "tests/files/not_existing.png",
            "target/test1.jpg",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
