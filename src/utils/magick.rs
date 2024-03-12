use magick_rust::{magick_wand_genesis, MagickWand};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.

static START: Once = Once::new();

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

pub fn optimize(config: &Config) -> Result<(), magick_rust::MagickError> {
    println!(
        "MAGICK: Resizing and auto orienting image: {} -> {}",
        config.input_path, config.output_path
    );

    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(config.input_path)?;
    wand.auto_orient();
    wand.strip_image()?;
    let width = config.width.unwrap_or(0);
    let height = config.height.unwrap_or(0);
    wand.fit(width as usize, height as usize);
    wand.set_image_compression_quality(75)?;

    wand.write_image(config.output_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn magic_resize_and_auto_orient() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: "tests/files/orientation_test.jpg",
            output_path: "target/magick_out.jpg",
            width: Some(240),
            height: Some(100),
        })
    }
    #[test]
    fn magic_resize_and_auto_orient_gif() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: "tests/files/test1.gif",
            output_path: "target/magick_out.gif",
            width: Some(100),
            height: Some(100),
        })
    }
}
