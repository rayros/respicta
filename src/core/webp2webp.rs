use crate::{
    utils::webp::{self, WebPError},
    Dimensions, InputOutput,
};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), WebPError>
where
    T: InputOutput + Dimensions,
{
    webp::optimize(config)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn webp2webp() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/webp2webp_test1.webp".into(),
            output_path: &"target/webp2webp_test1.webp".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
