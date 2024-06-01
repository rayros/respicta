use crate::{
    utils::webp::{self, Error},
    Dimensions, PathAccessor, Quality,
};

/// # Errors
///
/// Returns an error if the conversion fails.
///
pub fn convert<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions + Quality,
{
    webp::optimize(config)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn png2webp() {
        use super::*;

        convert(&Config::new(
            "tests/files/png2webp_test1.png",
            "target/png2webp_test1.webp",
            Some(100),
            None,
        ))
        .unwrap();
    }
}
