#![allow(clippy::cast_precision_loss)]

use magick_rust::{magick_wand_genesis, MagickWand};
use std::{fs::create_dir_all, sync::Once};
use thiserror::Error;

use crate::{Dimensions, PathAccessor};

use super::fit;

static START: Once = Once::new();

#[derive(Debug, Error)]
pub enum Error {
    #[error("Magick({0})")]
    Magick(magick_rust::MagickError),
    #[error("Io({0})")]
    Io(std::io::Error),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T, filter: Option<magick_rust::FilterType>) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(&config.input_path().display().to_string())
        .map_err(Error::Magick)?;

    let width = config
        .width()
        .map_or(wand.get_image_width(), |s| s as usize);
    let height = config
        .height()
        .map_or(wand.get_image_height(), |s| s as usize);

    wand.auto_orient();
    wand.strip_image().map_err(Error::Magick)?;

    let (new_width, new_height) = fit(
        u32::try_from(wand.get_image_width()).map_err(Error::TryFromIntError)?,
        u32::try_from(wand.get_image_height()).map_err(Error::TryFromIntError)?,
        u32::try_from(width).map_err(Error::TryFromIntError)?,
        u32::try_from(height).map_err(Error::TryFromIntError)?,
    );

    if let Some(filter) = filter {
        wand.resize_image(new_width as usize, new_height as usize, filter);
    } else {
        wand.adaptive_resize_image(new_width as usize, new_height as usize)
            .map_err(Error::Magick)?;
    }

    wand.set_image_compression_quality(75)
        .map_err(Error::Magick)?;

    if let Some(parent) = config.output_path().parent() {
        create_dir_all(parent).map_err(Error::Io)?;
    }

    wand.write_image(&config.output_path().display().to_string())
        .map_err(Error::Magick)
}

#[cfg(test)]
mod tests {
    use magick_rust::bindings::FilterType_LanczosFilter;

    use crate::Config;

    #[test]
    fn magic_resize_and_auto_orient() {
        use super::*;

        optimize(
            &Config::new(
                "tests/files/orientation_test.jpg",
                "target/magick_out.jpg",
                Some(240),
                Some(100),
            ),
            Some(FilterType_LanczosFilter),
        )
        .unwrap();
    }

    #[test]
    fn magic_resize_and_auto_orient_gif() {
        use super::*;

        optimize(
            &Config::new(
                "tests/files/test1.gif",
                "target/magick_out.gif",
                Some(100),
                Some(100),
            ),
            None,
        )
        .unwrap();
    }

    #[test]
    fn nested_dir() {
        use super::*;

        optimize(
            &Config::new(
                "tests/files/orientation_test.jpg",
                "target/magick_nested/magick_out.jpg",
                Some(240),
                Some(100),
            ),
            Some(FilterType_LanczosFilter),
        )
        .unwrap();
    }
}
