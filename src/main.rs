use oxipng::{optimize, InFile, Options, OutFile};

fn main() {
    // Specify the input PNG file path
    let input_png_path = "tests/files/issue-159.png";

    // Specify the output PNG file path (optional)
    let output_png_path = "output-image.png";

    // Set up the optimization options
    let options = Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    // Perform PNG optimization
    match optimize(
        &input_png_path.into(),
        &OutFile::from_path(output_png_path.into()),
        &options,
    ) {
        Ok(_) => {
            println!("Oxipng optimization successful!");
        }
        Err(e) => {
            eprintln!("Error running oxipng: {:?}", e);
        }
    }
}
