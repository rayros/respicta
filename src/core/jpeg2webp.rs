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
    fn jpeg2webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/jpeg2webp_test1.jpeg".into(),
            output_path: &"target/jpeg2webp_test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
