use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.

static START: Once = Once::new();

pub fn resize_and_auto_orient(
    input_jpg_path: &str,
    output_jpg_path: &str,
    target_width: usize,
    target_height: usize,
) -> Result<(), magick_rust::MagickError> {
    use magick_rust::{magick_wand_genesis, MagickWand};

    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(input_jpg_path)?;
    wand.auto_orient();
    wand.strip_image()?;
    wand.fit(target_width, target_height);
    wand.set_image_compression_quality(75)?;
    wand.write_image(output_jpg_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn magic_resize_and_auto_orient() -> Result<(), magick_rust::MagickError> {
        use super::*;
        // Specify the input PNG file path
        let input_jpg_path = "tests/files/img20230418182427.jpg";

        // Specify the output PNG file path (optional)
        let output_png_path = "target/magick_out.jpg";

        resize_and_auto_orient(input_jpg_path, output_png_path, 240, 100)
    }
    #[test]
    fn magic_resize_and_auto_orient_gif() -> Result<(), magick_rust::MagickError> {
        use super::*;
        // Specify the input PNG file path
        let input_jpg_path = "tests/files/test1.gif";

        // Specify the output PNG file path (optional)
        let output_png_path = "target/magick_out.gif";

        resize_and_auto_orient(input_jpg_path, output_png_path, 500, 0)
    }
}
