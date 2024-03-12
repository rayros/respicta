use crate::utils::gifsicle;

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
}

pub fn convert(config: &Config) -> std::result::Result<(), std::io::Error> {
    gifsicle::optimize(gifsicle::Config {
        input_path: config.input_path,
        output_path: config.output_path,
        width: config.width,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn gif2gif() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/gif2gif_test1.gif",
            output_path: "target/gif2gif_test1.gif",
            width: Some(100),
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn gif2gif_panic() {
        use super::*;

        convert(&Config {
            input_path: "tests/files/gif2gif_notexisting_test1.gif",
            output_path: "target/gif2gif_test1.gif",
            width: Some(100),
        })
        .unwrap();
    }
}
