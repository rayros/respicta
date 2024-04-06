pub mod core;
pub mod extensions;
pub mod server;
pub mod utils;

use core::{gif2gif, gif2webp, jpeg2jpeg, jpeg2webp, png2jpeg, png2png, png2webp, webp2webp};
use extensions::{GIF, JFIF, JPEG, JPG, PNG, WEBP};
use std::path::PathBuf;

pub trait InputOutput {
    fn input_path(&self) -> &PathBuf;
    fn output_path(&self) -> &PathBuf;
}

pub trait Dimensions {
    fn width(&self) -> Option<u32>;
    fn height(&self) -> Option<u32>;
}

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl InputOutput for Config<'_> {
    fn input_path(&self) -> &PathBuf {
        self.input_path
    }

    fn output_path(&self) -> &PathBuf {
        self.output_path
    }
}

impl Dimensions for Config<'_> {
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

        let config = Config {
            input_path: &"tests/files/test1.jpg".into(),
            output_path: &"target/test1.jpg".into(),
            width: Some(100),
            height: None,
        };

        assert_eq!(config.input_path(), &PathBuf::from("tests/files/test1.jpg"));
        assert_eq!(config.output_path(), &PathBuf::from("target/test1.jpg"));
        assert_eq!(config.width(), Some(100));
        assert_eq!(config.height(), None);
    }

    #[test]
    fn convert() -> anyhow::Result<()> {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/orientation_test.jpg".into(),
            output_path: &"target/convert_test1.webp".into(),
            width: Some(100),
            height: None,
        })?;

        convert(&Config {
            input_path: &"tests/files/orientation_test.jpeg".into(),
            output_path: &"target/convert_test2.webp".into(),
            width: Some(100),
            height: None,
        })?;

        convert(&Config {
            input_path: &"tests/files/convert_test1.png".into(),
            output_path: &"target/convert_test3.webp".into(),
            width: None,
            height: None,
        })?;

        convert(&Config {
            input_path: &"tests/files/convert_test1.png".into(),
            output_path: &"target/convert_test4.webp".into(),
            width: Some(10),
            height: None,
        })?;

        convert(&Config {
            input_path: &"tests/files/convert_test1.png".into(),
            output_path: &"target/convert_test5.webp".into(),
            width: None,
            height: Some(10),
        })?;

        convert(&Config {
            input_path: &"tests/files/convert_test1.gif".into(),
            output_path: &"target/convert_test6.gif".into(),
            width: Some(10),
            height: Some(10),
        })?;

        Ok(())
    }

    #[test]
    fn convert_jfif_to_webp() -> anyhow::Result<()> {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/convert_test_jfif.jfif".into(),
            output_path: &"target/convert_test7.webp".into(),
            width: Some(500),
            height: None,
        })?;

        Ok(())
    }

    #[test]
    fn survive_extension_wrong_format_jpg_to_webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/convert_test2.jpg".into(),
            output_path: &"target/convert_test8.webp".into(),
            width: Some(500),
            height: None,
        })
        .unwrap();
    }

    #[test]
    fn extension_in_uppercase() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/convert_test1.JPG".into(),
            output_path: &"target/convert_test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Unsupported conversion: jpg -> tiff"]
    fn convert_panic() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.jpg".into(),
            output_path: &"target/test1.tiff".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting jpg to webp"]
    fn convert_panic_jpg_to_webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.jpg".into(),
            output_path: &"target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting jpg to jpg"]
    fn convert_panic_jpg_to_jpg() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.jpg".into(),
            output_path: &"target/test1.jpg".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting png to png"]
    fn convert_panic_png_to_png() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.png".into(),
            output_path: &"target/test1.png".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting webp to webp"]
    fn convert_panic_webp_to_webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.webp".into(),
            output_path: &"target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Input file has no extension"]
    fn convert_panic_no_input_extension() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing".into(),
            output_path: &"target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Output file has no extension"]
    fn convert_panic_no_output_extension() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.jpg".into(),
            output_path: &"target/test1".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting gif to gif"]
    fn convert_panic_gif_to_gif() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.gif".into(),
            output_path: &"target/test1.gif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting gif to webp"]
    fn convert_panic_gif_to_webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.gif".into(),
            output_path: &"target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting png to webp"]
    fn convert_panic_png_to_webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.png".into(),
            output_path: &"target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Error converting png to jpg"]
    fn convert_panic_png_to_jpg() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/not_existing.png".into(),
            output_path: &"target/test1.jpg".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
