use crate::{
    utils::webp::{self, Error},
    Dimensions, PathAccessor,
};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    webp::optimize(config)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/png2webp_test1.png".into(),
            output_path: &"target/png2webp_test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
