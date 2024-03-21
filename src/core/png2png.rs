use std::{error::Error, path::PathBuf};

use crate::utils::{magick, oxipng};

use super::gif2webp::path_without_extension;

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn convert(config: &Config) -> Result<(), Box<dyn Error>> {
    let output_path_without_extension = path_without_extension(config.output_path).unwrap();
    let step1_output = &PathBuf::from(format!("{output_path_without_extension}_step1.png"));

    magick::optimize(&magick::Config {
        input_path: config.input_path,
        output_path: step1_output,
        width: config.width,
        height: config.height,
    })?;
    oxipng::optimize(&oxipng::Config {
        input_path: step1_output,
        output_path: config.output_path,
    })?;
    std::fs::remove_file(step1_output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn png2png() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/png2png_test1.png"),
            output_path: &PathBuf::from("target/png2png_test1.png"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn png2png_panic() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/png2png_notexisting_test1.png"),
            output_path: &PathBuf::from("target/png2png_test1.png"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
