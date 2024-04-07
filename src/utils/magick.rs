#![allow(clippy::cast_precision_loss)]

use magick_rust::{magick_wand_genesis, MagickWand};
use std::{path::PathBuf, sync::Once};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.

static START: Once = Once::new();

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize(config: &Config) -> Result<(), magick_rust::MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(&config.input_path.display().to_string())?;

    let width = config.width.map_or(wand.get_image_width(), |s| s as usize);
    let height = config
        .height
        .map_or(wand.get_image_height(), |s| s as usize);

    wand.auto_orient();
    wand.strip_image()?;

    let mut width_ratio = width as f64;
    width_ratio /= wand.get_image_width() as f64;
    let mut height_ratio = height as f64;
    height_ratio /= wand.get_image_height() as f64;
    let (new_width, new_height) = if width_ratio < height_ratio {
        (
            width,
            (wand.get_image_height() as f64 * width_ratio) as usize,
        )
    } else {
        (
            (wand.get_image_width() as f64 * height_ratio) as usize,
            height,
        )
    };

    wand.adaptive_resize_image(new_width, new_height)?;

    wand.set_image_compression_quality(75)?;

    wand.write_image(&config.output_path.display().to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn magic_resize_and_auto_orient() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: &PathBuf::from("tests/files/orientation_test.jpg"),
            output_path: &PathBuf::from("target/magick_out.jpg"),
            width: Some(240),
            height: Some(100),
        })
    }
    #[test]
    fn magic_resize_and_auto_orient_gif() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: &PathBuf::from("tests/files/test1.gif"),
            output_path: &PathBuf::from("target/magick_out.gif"),
            width: Some(100),
            height: Some(100),
        })
    }
}
