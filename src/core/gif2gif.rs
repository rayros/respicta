use crate::utils::gifsicle;
use std::path::PathBuf;

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn convert(config: &Config) -> std::result::Result<(), std::io::Error> {
    gifsicle::optimize(gifsicle::Config {
        input_path: config.input_path,
        output_path: config.output_path,
        width: config.width,
        height: config.height,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn gif2gif() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/gif2gif_test1.gif"),
            output_path: &PathBuf::from("target/gif2gif_test1.gif"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn gif2gif_panic() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/gif2gif_notexisting_test1.gif"),
            output_path: &PathBuf::from("target/gif2gif_test1.gif"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
