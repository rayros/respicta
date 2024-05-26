#[cfg(feature = "server-app-error")]
pub mod app_error;
#[cfg(feature = "command-server")]
pub mod command_server;
pub mod core;
pub mod extensions;
#[cfg(feature = "web-service")]
pub mod server;
pub mod utils;

use core::{gif2gif, gif2webp, jpeg2jpeg, jpeg2webp, png2jpeg, png2png, png2webp, webp2webp};
use derive_builder::Builder;
use extensions::{GIF, JFIF, JPEG, JPG, PNG, WEBP};
use std::path::{Path, PathBuf};
use thiserror::Error;
use utils::{gifsicle, magick, webp};

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
    Png2Jpeg(magick::Error),
    #[error("Error converting png to webp: {0}")]
    Png2Webp(webp::Error),
    #[error("Error converting jpg to jpg: {0}")]
    Jpeg2Jpeg(magick::Error),
    #[error("Error converting jpg to webp: {0}")]
    Jpeg2Webp(webp::Error),
    #[error("Error converting gif to gif: {0}")]
    Gif2Gif(gifsicle::Error),
    #[error("Error converting gif to webp: {0}")]
    Gif2Webp(gif2webp::Error),
    #[error("Error converting webp to webp: {0}")]
    Webp2Webp(webp::Error),
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
            match (input_extension.as_str(), output_extension.as_str()) {
                (GIF, GIF) => gif2gif::convert(config).map_err(Error::Gif2Gif),
                (GIF, WEBP) => gif2webp::convert(config).map_err(Error::Gif2Webp),
                (PNG, WEBP) => png2webp::convert(config).map_err(Error::Png2Webp),
                (WEBP, WEBP) => webp2webp::convert(config).map_err(Error::Webp2Webp),
                (JPG | JPEG | JFIF, WEBP) => jpeg2webp::convert(config).map_err(Error::Jpeg2Webp),
                (JPG | JPEG | JFIF, JPG | JPEG | JFIF) => {
                    jpeg2jpeg::convert(config).map_err(Error::Jpeg2Jpeg)
                }
                (PNG, PNG) => png2png::convert(config).map_err(Error::Png2Png),
                (PNG, JPG | JPEG | JFIF) => png2jpeg::convert(config).map_err(Error::Png2Jpeg),
                (input_extension, output_extension) => Err(Error::UnsupportedConversion(
                    input_extension.to_string(),
                    output_extension.to_string(),
                )),
            }
        }
        (None, _) => Err(Error::InputFileHasNoExtension),
        (_, None) => Err(Error::OutputFileHasNoExtension),
    }
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
    #[should_panic = "Jpeg2Webp(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))"]
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
    #[should_panic = "Jpeg2Jpeg(Magick(MagickError(\"failed to read image\")))"]
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
    #[should_panic = "Png2Png(Magick(Magick(MagickError(\"failed to read image\"))))"]
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
    #[should_panic = "Webp2Webp(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))"]
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
    #[should_panic = "Png2Webp(Io(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))"]
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
    #[should_panic = "Png2Jpeg(Magick(MagickError(\"failed to read image\")))"]
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
