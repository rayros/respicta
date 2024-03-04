use magick_rust::{magick_wand_genesis, MagickWand};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

pub fn resize_and_auto_orient(
    input_jpg_path: &str,
    width: usize,
    output_jpg_path: &str,
) -> Result<(), magick_rust::MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(input_jpg_path)?;
    wand.auto_orient();
    wand.fit(width, wand.get_image_height());
    wand.write_image(output_jpg_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn magick_resize_and_auto_orient() -> Result<(), magick_rust::MagickError> {
        use super::*;
        // Specify the input PNG file path
        let input_jpg_path = "tests/files/orientation_test.jpg";

        // Specify the output PNG file path (optional)
        let output_png_path = "target/out_image_1.jpg";

        resize_and_auto_orient(input_jpg_path, 240, output_png_path)
    }
}
