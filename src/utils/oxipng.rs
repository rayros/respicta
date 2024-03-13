use oxipng::{Options, OutFile};

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
}

pub fn optimize(config: &Config) -> Result<(), oxipng::PngError> {
    println!(
        "OXIPNG: Optimizing PNG file: {} -> {}",
        config.input_path, config.output_path
    );
    // Set up the optimization options
    let options = Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    // Perform PNG optimization
    oxipng::optimize(
        &config.input_path.into(),
        &OutFile::from_path(config.output_path.into()),
        &options,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn oxipng_optimize() {
        use super::*;

        optimize(&Config {
            input_path: "tests/files/issue-159.png",
            output_path: "target/issue-159.png",
        })
        .unwrap();
    }
}
