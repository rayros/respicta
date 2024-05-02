use crate::{
    utils::gifsicle::{self, Error},
    Dimensions, PathAccessor,
};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> std::result::Result<(), Error>
where
    T: PathAccessor + Dimensions,
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
    #[should_panic = "Exit(1)"]
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
