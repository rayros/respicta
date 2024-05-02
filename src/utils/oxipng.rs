use oxipng::{Options, OutFile};

use crate::PathAccessor;

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T) -> Result<(), oxipng::PngError>
where
    T: PathAccessor,
{
    let input = &config.input_path().into();
    let output = &OutFile::from_path(config.output_path().into());
    let options = &Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    oxipng::optimize(input, output, options)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn oxipng_optimize() {
        use super::*;

        optimize(&Config::new(
            "tests/files/issue-159.png",
            "target/issue-159.png",
            None,
            None,
        ))
        .unwrap();
    }
}
