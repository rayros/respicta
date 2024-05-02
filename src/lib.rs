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
use extensions::{GIF, JFIF, JPEG, JPG, PNG, WEBP};
use std::path::{Path, PathBuf};

pub trait PathAccessor {
    fn input_path(&self) -> &PathBuf;
    fn output_path(&self) -> &PathBuf;
}

pub trait Dimensions {
    fn width(&self) -> Option<u32>;
    fn height(&self) -> Option<u32>;
}

pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
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

/// # Errors
///
/// Returns an error if:
/// * The input file path has no extension
/// * The output file path has no extension
/// * The conversion is not supported
/// * An error occurs during the conversion
///
pub fn convert(config: &Config) -> anyhow::Result<()> {
    match (
        config
            .input_path()
            .extension()
            // to lower case
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
                (GIF, GIF) => gif2gif::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting gif to gif: {:?}", e)),
                (GIF, WEBP) => gif2webp::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting gif to webp: {:?}", e)),
                (PNG, WEBP) => png2webp::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting png to webp: {:?}", e)),
                (WEBP, WEBP) => webp2webp::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting webp to webp: {:?}", e)),
                (JPG | JPEG | JFIF, WEBP) => jpeg2webp::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting jpg to webp: {:?}", e)),
                (JPG | JPEG | JFIF, JPG | JPEG | JFIF) => jpeg2jpeg::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting jpg to jpg: {:?}", e)),
                (PNG, PNG) => png2png::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting png to png: {:?}", e)),
                (PNG, JPG | JPEG | JFIF) => png2jpeg::convert(config)
                    .map_err(|e| anyhow::anyhow!("Error converting png to jpg: {:?}", e)),
                (input_extension, output_extension) => {
                    anyhow::bail!("Unsupported conversion: {input_extension} -> {output_extension}",)
                }
            }
        }
        (None, _) => anyhow::bail!("Input file has no extension"),
        (_, None) => anyhow::bail!("Output file has no extension"),
    }
}

#[cfg(test)]
mod tests {
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
    fn convert() -> anyhow::Result<()> {
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
    fn convert_jfif_to_webp() -> anyhow::Result<()> {
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
    #[should_panic = "Unsupported conversion: jpg -> tiff"]
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
    #[should_panic = "Error converting jpg to webp"]
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
    #[should_panic = "Error converting jpg to jpg"]
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
    #[should_panic = "Error converting png to png"]
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
    #[should_panic = "Error converting webp to webp"]
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
    #[should_panic = "Input file has no extension"]
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
    #[should_panic = "Output file has no extension"]
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
    #[should_panic = "Error converting gif to gif"]
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
    #[should_panic = "Error converting gif to webp"]
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
    #[should_panic = "Error converting png to webp"]
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
    #[should_panic = "Error converting png to jpg"]
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
