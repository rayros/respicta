use std::{
    fs::{create_dir_all, write},
    process::Command,
};
use thiserror::Error;

use crate::{Dimensions, PathAccessor};
use image::{io::Reader, GenericImageView};
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPEncodingError, WebPMemoryWrite, WebPMemoryWriter,
    WebPMemoryWriterClear, WebPMemoryWriterInit, WebPPicture, WebPPictureFree,
    WebPPictureImportRGBA, WebPPictureRescale, WebPValidateConfig,
};

use super::fit;

pub struct RGBAImage {
    pub data: *const u8,
    pub width: u32,
    pub height: u32,
}

pub struct NewDimensions {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Dimensions for NewDimensions {
    fn width(&self) -> Option<u32> {
        self.width
    }

    fn height(&self) -> Option<u32> {
        self.height
    }
}

impl NewDimensions {
    fn fit_dimensions<T>(image: &RGBAImage, config: &T) -> NewDimensions
    where
        T: Dimensions,
    {
        let (new_width, new_height) = fit(
            image.width,
            image.height,
            config.width().unwrap_or(image.width),
            config.height().unwrap_or(image.height),
        );

        NewDimensions {
            width: Some(new_width),
            height: Some(new_height),
        }
    }
}

#[derive(Debug, Error)]
pub enum LibWebPError {
    #[error("Failed to initialize WebP config")]
    ConfigInit(()),
    #[error("Failed to validate WebP config")]
    ConfigValidate,
    #[error("Failed to initialize WebP picture")]
    Picture(()),
    #[error("Failed to encode WebP image: {0:?}")]
    Encoding(WebPEncodingError),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

fn rgba_to_webp<T>(image: &RGBAImage, config: &T) -> Result<Vec<u8>, LibWebPError>
where
    T: Dimensions,
{
    let mut webp_config = WebPConfig::new().map_err(LibWebPError::ConfigInit)?;
    webp_config.lossless = 1;
    webp_config.alpha_compression = 1;
    webp_config.quality = 1.0;

    let mut picture = WebPPicture::new().map_err(LibWebPError::Picture)?;
    picture.use_argb = 1;
    picture.width = i32::try_from(image.width).map_err(LibWebPError::TryFromIntError)?;
    picture.height = i32::try_from(image.height).map_err(LibWebPError::TryFromIntError)?;

    let mut ww: ::core::mem::MaybeUninit<WebPMemoryWriter> = ::core::mem::MaybeUninit::uninit();
    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr().cast::<std::ffi::c_void>();

    unsafe {
        if WebPValidateConfig(&webp_config) == 0 {
            return Err(LibWebPError::ConfigValidate);
        }

        let memory_writer_ptr = ww.as_mut_ptr();

        WebPMemoryWriterInit(memory_writer_ptr);

        let rgba_stride = i32::try_from(image.width * 4).map_err(LibWebPError::TryFromIntError)?;

        WebPPictureImportRGBA(&mut picture, image.data, rgba_stride);

        let target_width = config
            .width()
            .and_then(|w| i32::try_from(w).ok())
            .unwrap_or(0);

        let target_height = config
            .height()
            .and_then(|h| i32::try_from(h).ok())
            .unwrap_or(0);

        WebPPictureRescale(&mut picture, target_width, target_height);

        let encode_result = WebPEncode(&webp_config, &mut picture);

        if encode_result == VP8StatusCode::VP8_STATUS_OK as i32 {
            return Err(LibWebPError::Encoding(picture.error_code));
        }

        let memory_writer = ww.assume_init();
        let contents = std::slice::from_raw_parts(memory_writer.mem, memory_writer.size).to_vec();

        WebPPictureFree(&mut picture);
        WebPMemoryWriterClear(memory_writer_ptr);

        Ok(contents)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    LibWebPError(#[from] LibWebPError),
}

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    let input_image = Reader::open(config.input_path())
        .map_err(Error::Io)?
        .with_guessed_format()
        .map_err(Error::Io)?
        .decode()
        .map_err(Error::Image)?;

    let dimensions = input_image.dimensions();
    let dimension_width = dimensions.0;
    let dimension_height = dimensions.1;
    let rgba_image = input_image.into_rgba8();

    let rgba_image = RGBAImage {
        data: rgba_image.as_ptr(),
        width: dimension_width,
        height: dimension_height,
    };

    let new_dimensions = NewDimensions::fit_dimensions(&rgba_image, config);

    let contents = rgba_to_webp(&rgba_image, &new_dimensions).map_err(Error::LibWebPError)?;

    if let Some(parent) = config.output_path().parent() {
        create_dir_all(parent).map_err(Error::Io)?;
    }

    write(config.output_path(), contents).map_err(Error::Io)?;

    Ok(())
}

fn process_exit_code(code: Option<i32>) -> std::result::Result<(), std::io::Error> {
    match code {
        Some(0) => Ok(()),
        Some(_) | None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "gif2webp failed",
        )),
    }
}

/// # Errors
///
/// Returns an error if the command gif2webp fails.
///
pub fn optimize_gif<T>(config: &T) -> Result<(), std::io::Error>
where
    T: PathAccessor,
{
    let input_path = config.input_path().display();
    let output_path = config.output_path().display();
    let command = format!("gif2webp -o {output_path} -q 75 -m 6 -mt -v {input_path}");
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let code = output.status.code();

    process_exit_code(code)
}

#[cfg(test)]
mod tests {
    use crate::{core::PathIO, Config};

    #[test]
    fn webp_optimize_png_to_webp() {
        use super::*;

        optimize(&Config::new(
            "tests/files/issue-159.png",
            "target/issue-159.webp",
            Some(100),
            Some(100),
        ))
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_static() {
        use super::*;

        optimize(&Config::new(
            "tests/files/test1.gif",
            "target/gif_test1_static.webp",
            Some(100),
            Some(100),
        ))
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_2() {
        use super::*;

        optimize_gif(&PathIO::new(
            &"tests/files/test1.gif".into(),
            &"target/webp_gif_test1.webp".into(),
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gif2webp failed\" }"]
    fn webp_optimize_gif_to_webp_panic() {
        use super::*;

        optimize_gif(&PathIO::new(
            &"tests/files/not_existing.gif".into(),
            &"target/webp_gif_test1.webp".into(),
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gif2webp failed\" }"]
    fn process_exit_code_terminated_by_signal_panic() {
        use super::*;

        process_exit_code(None).unwrap();
    }
}
