use image::io::Reader;
use std::path::PathBuf;
use thiserror::Error;

use crate::{Dimensions, InputOutput};
use image::GenericImageView;
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPEncodingError, WebPMemoryWrite, WebPMemoryWriter,
    WebPMemoryWriterClear, WebPMemoryWriterInit, WebPPicture, WebPPictureFree,
    WebPPictureImportRGBA, WebPPictureRescale, WebPValidateConfig,
};

pub struct RGBAImage {
    pub data: *const u8,
    pub width: i32,
    pub height: i32,
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
            image.width as u32,
            image.height as u32,
            config.width().unwrap_or(image.width as u32),
            config.height().unwrap_or(image.height as u32),
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
    picture.width = image.width;
    picture.height = image.height;

    let mut ww: ::core::mem::MaybeUninit<WebPMemoryWriter> = ::core::mem::MaybeUninit::uninit();
    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr().cast::<std::ffi::c_void>();

    unsafe {
        if WebPValidateConfig(&webp_config) == 0 {
            return Err(LibWebPError::ConfigValidate);
        }

        let memory_writer_ptr = ww.as_mut_ptr();
        WebPMemoryWriterInit(memory_writer_ptr);
        WebPPictureImportRGBA(&mut picture, image.data, image.width * 4);

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

fn fit(width: u32, height: u32, max_width: u32, max_height: u32) -> (u32, u32) {
    let width_ratio = f64::from(max_width) / f64::from(width);
    let height_ratio = f64::from(max_height) / f64::from(height);

    if width_ratio < height_ratio {
        (max_width, (f64::from(height) * width_ratio) as u32)
    } else {
        ((f64::from(width) * height_ratio) as u32, max_height)
    }
}

#[derive(Debug, Error)]
pub enum WebPError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    LibWebPError(#[from] LibWebPError),
}

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T) -> Result<(), WebPError>
where
    T: InputOutput + Dimensions,
{
    let input_image = Reader::open(config.input_path())
        .map_err(WebPError::Io)?
        .with_guessed_format()
        .map_err(WebPError::Io)?
        .decode()
        .map_err(WebPError::Image)?;

    let dimensions = input_image.dimensions();
    let dimension_width = i32::try_from(dimensions.0).map_err(WebPError::TryFromIntError)?;
    let dimension_height = i32::try_from(dimensions.1).map_err(WebPError::TryFromIntError)?;
    let rgba_image = input_image.into_rgba8();

    let rgba_image = RGBAImage {
        data: rgba_image.as_ptr(),
        width: dimension_width,
        height: dimension_height,
    };

    let new_dimensions = NewDimensions::fit_dimensions(&rgba_image, config);

    let contents = rgba_to_webp(&rgba_image, &new_dimensions).map_err(WebPError::LibWebPError)?;

    if let Some(parent) = config.output_path().parent() {
        std::fs::create_dir_all(parent).map_err(WebPError::Io)?;
    }

    std::fs::write(config.output_path(), contents).map_err(WebPError::Io)?;

    Ok(())
}

pub struct GifConfig<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
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

pub fn optimize_gif(config: &GifConfig) -> std::result::Result<(), std::io::Error> {
    let input_path = config.input_path.display();
    let output_path = config.output_path.display();
    let command = format!("gif2webp -o {output_path} -q 75 -m 6 -mt -v {input_path}",);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let code = output.status.code();

    process_exit_code(code)
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn webp_optimize_png_to_webp() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/issue-159.png".into(),
            output_path: &"target/issue-159.webp".into(),
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_static() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/test1.gif".into(),
            output_path: &"target/gif_test1_static.webp".into(),
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_2() {
        use super::*;

        optimize_gif(&GifConfig {
            input_path: &"tests/files/test1.gif".into(),
            output_path: &"target/webp_gif_test1.webp".into(),
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gif2webp failed\" }"]
    fn webp_optimize_gif_to_webp_panic() {
        use super::*;

        optimize_gif(&GifConfig {
            input_path: &"tests/files/not_existing.gif".into(),
            output_path: &"target/webp_gif_test1.webp".into(),
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Custom { kind: Other, error: \"gif2webp failed\" }"]
    fn process_exit_code_terminated_by_signal_panic() {
        use super::*;

        process_exit_code(None).unwrap();
    }
}
