use crate::{utils::gifsicle, Dimensions, InputOutput};

/// # Errors
///
/// Returns an error if the conversion fails.
pub fn convert<T>(config: &T) -> std::result::Result<(), std::io::Error>
where
    T: InputOutput + Dimensions,
{
    gifsicle::optimize(config)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn gif2gif() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/gif2gif_test1.gif".into(),
            output_path: &"target/gif2gif_test1.gif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gifsicle failed\" }"]
    fn gif2gif_panic() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/gif2gif_notexisting_test1.gif".into(),
            output_path: &"target/gif2gif_test1.gif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
