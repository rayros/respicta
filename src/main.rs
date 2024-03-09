use image_resizer::utils::gifsicle;
use image_resizer::utils::magick;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    magick::resize_and_auto_orient(
        "tests/files/orientation_test.jpg",
        "target/magick_out_image_1.jpg",
        240,
        100,
    )?;
    let input_path = "tests/files/test1.gif";

    // Specify the output PNG file path (optional)
    let output_path = "target/gifsicle_test1.gif";

    gifsicle::optimize(&gifsicle::Config {
        input_path,
        output_path,
        width: Some(100),
    });

    // Specify the input PNG file path
    // optimize(input_png_path, output_png_path)

    Ok(())
}
