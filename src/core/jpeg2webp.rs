use crate::{utils::webp, Dimensions, InputOutput};

pub fn convert<T>(config: &T) -> anyhow::Result<()>
where
    T: InputOutput + Dimensions,
{
    webp::optimize(config, image::ImageFormat::Jpeg)
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
