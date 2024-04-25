use crate::{utils::webp, Dimensions, InputOutput};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> anyhow::Result<()>
where
    T: InputOutput + Dimensions,
{
    Ok(webp::optimize(config)?)
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
