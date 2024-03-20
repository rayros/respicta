pub mod core;
pub mod utils;

use std::path::PathBuf;

use crate::core::{gif2gif, gif2webp, jpeg2jpeg, jpeg2webp, png2png, png2webp, webp2webp};

pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

fn resolve_extension(path: &PathBuf) -> &str {
    let path = path.to_str().unwrap();
    let extension = path.split('.').last().unwrap();
    extension
}

pub fn convert(config: &Config) -> anyhow::Result<()> {
    let input_extension = resolve_extension(&config.input_path);
    let output_extension = resolve_extension(&config.output_path);
    let input_path = config.input_path.to_str().unwrap();
    let output_path = config.output_path.to_str().unwrap();
    let width = config.width;
    let height = config.height;
    match (input_extension, output_extension) {
        ("gif", "gif") => gif2gif::convert(&gif2gif::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting gif to gif: {:?}", e)),
        ("gif", "webp") => gif2webp::convert(&gif2webp::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting gif to webp: {:?}", e)),
        ("png", "webp") => png2webp::convert(&png2webp::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting png to webp: {:?}", e)),
        ("webp", "webp") => webp2webp::convert(&webp2webp::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting webp to webp: {:?}", e)),
        ("jpg" | "jpeg" | "jfif", "webp") => jpeg2webp::convert(&jpeg2webp::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting jpg to webp: {:?}", e)),
        ("jpg" | "jpeg" | "jfif", "jpg" | "jpeg" | "jfif") => {
            jpeg2jpeg::convert(&jpeg2jpeg::Config {
                input_path,
                output_path,
                width,
                height,
            })
            .map_err(|e| anyhow::anyhow!("Error converting jpg to jpg: {:?}", e))
        }
        ("png", "png") => png2png::convert(&png2png::Config {
            input_path,
            output_path,
            width,
            height,
        })
        .map_err(|e| anyhow::anyhow!("Error converting png to png: {:?}", e)),
        _ => anyhow::bail!(
            "Unsupported conversion: {} -> {}",
            input_extension,
            output_extension
        ),
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
