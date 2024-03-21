pub mod core;
pub mod extensions;
pub mod utils;
use core::{gif2gif, gif2webp, jpeg2jpeg, jpeg2webp, png2png, png2webp, webp2webp};
use std::path::PathBuf;

use extensions::{GIF, JFIF, JPEG, JPG, PNG, WEBP};

pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn convert(config: &Config) -> anyhow::Result<()> {
    let input_path = &config.input_path;
    let output_path = &config.output_path;
    let width = config.width;
    let height = config.height;

    match (
        input_path.extension().and_then(std::ffi::OsStr::to_str),
        output_path.extension().and_then(std::ffi::OsStr::to_str),
    ) {
        (Some(input_extension), Some(output_extension)) => {
            match (input_extension, output_extension) {
                (GIF, GIF) => gif2gif::convert(&gif2gif::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting gif to gif: {:?}", e)),
                (GIF, WEBP) => gif2webp::convert(&gif2webp::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting gif to webp: {:?}", e)),
                (PNG, WEBP) => png2webp::convert(&png2webp::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting png to webp: {:?}", e)),
                (WEBP, WEBP) => webp2webp::convert(&webp2webp::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting webp to webp: {:?}", e)),
                (JPG | JPEG | JFIF, WEBP) => jpeg2webp::convert(&jpeg2webp::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting jpg to webp: {:?}", e)),
                (JPG | JPEG | JFIF, JPG | JPEG | JFIF) => jpeg2jpeg::convert(&jpeg2jpeg::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting jpg to jpg: {:?}", e)),
                (PNG, PNG) => png2png::convert(&png2png::Config {
                    input_path,
                    output_path,
                    width,
                    height,
                })
                .map_err(|e| anyhow::anyhow!("Error converting png to png: {:?}", e)),
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
    fn convert() -> anyhow::Result<()> {
        use super::*;

        convert(&Config {
            input_path: "tests/files/orientation_test.jpg".into(),
            output_path: "target/test1.webp".into(),
            width: Some(100),
            height: None,
        })?;

        convert(&Config {
            input_path: "tests/files/orientation_test.jpeg".into(),
            output_path: "target/test1.webp".into(),
            width: Some(100),
            height: None,
        })?;

        Ok(())
    }

    #[test]
    #[should_panic = "Unsupported conversion: jpg -> tiff"]
    fn convert_panic() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/not_existing.jpg".into(),
            output_path: "target/test1.tiff".into(),
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
            input_path: "tests/files/not_existing.jpg".into(),
            output_path: "target/test1.webp".into(),
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
            input_path: "tests/files/not_existing.jpg".into(),
            output_path: "target/test1.jpg".into(),
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
            input_path: "tests/files/not_existing.png".into(),
            output_path: "target/test1.png".into(),
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
            input_path: "tests/files/not_existing.webp".into(),
            output_path: "target/test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
