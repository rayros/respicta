use std::path::PathBuf;

use crate::utils::webp;

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn convert(config: &Config) -> anyhow::Result<()> {
    webp::optimize(&webp::Config {
        input_path: config.input_path,
        output_path: config.output_path,
        width: config.width,
        height: config.height,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn webp2webp() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/webp2webp_test1.webp"),
            output_path: &PathBuf::from("target/webp2webp_test1.webp"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
