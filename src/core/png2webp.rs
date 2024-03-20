use crate::utils::webp;

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
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
    fn png2webp() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/png2webp_test1.png",
            output_path: "target/png2webp_test1.webp",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
