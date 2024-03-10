use oxipng::{Options, OutFile};

pub fn optimize(input_png_path: &str, output_png_path: &str) -> Result<(), oxipng::PngError> {
    println!(
        "OXIPNG: Optimizing PNG file: {} -> {}",
        input_png_path, output_png_path
    );
    // Set up the optimization options
    let options = Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    // Perform PNG optimization
    oxipng::optimize(
        &input_png_path.into(),
        &OutFile::from_path(output_png_path.into()),
        &options,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn oxipng_optimize() -> Result<(), oxipng::PngError> {
        use super::*;
        // Specify the input PNG file path
        let input_png_path = "tests/files/issue-159.png";

        // Specify the output PNG file path (optional)
        let output_png_path = "target/issue-159.png";

        optimize(input_png_path, output_png_path)
    }
}
