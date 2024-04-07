use std::error::Error;

use crate::utils::{magick, oxipng};

use crate::{Dimensions, InputOutput};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), Box<dyn Error>>
where
    T: InputOutput + Dimensions,
{
    let output_path = config.output_path();
    let step1_output_path = &output_path.with_extension("step1.png");
    let magick_config = magick::Config {
        input_path: config.input_path(),
        output_path: step1_output_path,
        width: config.width(),
        height: config.height(),
    };
    magick::optimize(&magick_config)?;
    let oxipng_config = oxipng::Config {
        input_path: step1_output_path,
        output_path: config.output_path(),
    };
    oxipng::optimize(&oxipng_config)?;
    std::fs::remove_file(step1_output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2png_test1() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/png2png_test1.png".into(),
            output_path: &"target/png2png_test1.png".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    fn png2png_test2() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/png2png_test2.png".into(),
            output_path: &"target/png2png_test2.png".into(),
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "MagickError(\"failed to read image\")"]
    fn png2png_panic() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/png2png_notexisting_test1.png".into(),
            output_path: &"target/png2png_test1.png".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
