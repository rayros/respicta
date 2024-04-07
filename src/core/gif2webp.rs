use crate::{
    utils::{gifsicle, webp},
    Config, Dimensions, InputOutput,
};

/// # Errors
///
/// Returns an error if the gifsicle command fails or if the input file does not exist.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), std::io::Error>
where
    T: InputOutput + Dimensions,
{
    let output_path = config.output_path();
    let step1_output_path = &output_path.with_extension("step1");
    let gifsicle_config = Config {
        input_path: config.input_path(),
        output_path: step1_output_path,
        width: config.width(),
        height: config.height(),
    };
    gifsicle::optimize(&gifsicle_config)?;
    let webp_config = webp::GifConfig {
        input_path: step1_output_path,
        output_path: config.output_path(),
    };
    webp::optimize_gif(&webp_config)?;
    std::fs::remove_file(step1_output_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn gif2webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/gif2webp_test1.gif".into(),
            output_path: &"target/gif2webp_test1.webp".into(),
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gifsicle failed\" }"]
    fn gif2webp_panic() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/gif2webp_notexisting_test1.gif".into(),
            output_path: &"target/gif2webp_test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
