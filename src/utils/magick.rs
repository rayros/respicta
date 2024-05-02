#![allow(clippy::cast_precision_loss)]

use magick_rust::{magick_wand_genesis, MagickWand};
use std::sync::Once;

use crate::{Dimensions, PathAccessor};

use super::fit;

static START: Once = Once::new();

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T) -> Result<(), magick_rust::MagickError>
where
    T: PathAccessor + Dimensions,
{
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(&config.input_path().display().to_string())?;

    let width = config
        .width()
        .map_or(wand.get_image_width(), |s| s as usize);
    let height = config
        .height()
        .map_or(wand.get_image_height(), |s| s as usize);

    wand.auto_orient();
    wand.strip_image()?;

    let (new_width, new_height) = fit(
        wand.get_image_width() as u32,
        wand.get_image_height() as u32,
        width as u32,
        height as u32,
    );

    wand.adaptive_resize_image(new_width as usize, new_height as usize)?;

    wand.set_image_compression_quality(75)?;

    wand.write_image(&config.output_path().display().to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Config;

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
