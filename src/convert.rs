use crate::core::{gif2webp, png2webp, webp2webp};

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

fn resolve_extension(path: &str) -> &str {
    let extension = path.split('.').last().unwrap();
    extension
}

pub fn convert(config: &Config) -> anyhow::Result<()> {
    let input_extension = resolve_extension(config.input_path);
    let output_extension = resolve_extension(config.output_path);
    let input_path = config.input_path;
    let output_path = config.output_path;
    let width = config.width;
    let height = config.height;
    match (input_extension, output_extension) {
        ("gif", "webp") => {
            gif2webp::convert(&gif2webp::Config {
                input_path,
                output_path,
                width,
            });
            Ok(())
        }
        ("png", "webp") => match png2webp::convert(&png2webp::Config {
            input_path,
            output_path,
            width,
            height,
        }) {
            Ok(()) => Ok(()),
            Err(e) => anyhow::bail!("Error converting png to webp: {:?}", e),
        },
        ("webp", "webp") => {
            match webp2webp::convert(&webp2webp::Config {
                input_path,
                output_path,
                width,
                height,
            }) {
                Ok(()) => Ok(()),
                Err(e) => anyhow::bail!("Error converting webp to webp: {:?}", e),
            }
        }
        _ => anyhow::bail!(
            "Unsupported conversion: {} -> {}",
            input_extension,
            output_extension
        ),
    }
}
