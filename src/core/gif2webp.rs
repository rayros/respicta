use crate::utils::gifsicle;
use crate::utils::webp;
pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[must_use]
pub fn path_without_extension(input_path: &str) -> String {
    let path = std::path::Path::new(input_path);
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let parent = path.parent().unwrap().to_str().unwrap();
    format!("{parent}/{file_stem}")
}

pub fn convert(config: &Config) -> std::result::Result<(), std::io::Error> {
    let step1_output = format!("{}_step1.gif", path_without_extension(config.output_path));
    gifsicle::optimize(gifsicle::Config {
        input_path: config.input_path,
        output_path: step1_output.as_str(),
        width: config.width,
        height: config.height,
    })?;
    webp::optimize_gif(&webp::GifConfig {
        input_path: step1_output.as_str(),
        output_path: config.output_path,
    })?;
    std::fs::remove_file(step1_output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn gif2webp() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/gif2webp_test1.gif",
            output_path: "target/gif2webp_test1.webp",
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn gif2webp_panic() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/gif2webp_panic_test1.gif",
            output_path: "target/gif2webp_test1.webp",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
