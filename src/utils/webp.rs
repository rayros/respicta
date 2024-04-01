use image::io::Reader as ImageReader;
use std::path::PathBuf;

use anyhow::{anyhow, bail};
use image::GenericImageView;
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPMemoryWrite, WebPMemoryWriter, WebPMemoryWriterInit,
    WebPPicture, WebPPictureFree, WebPPictureImportRGBA, WebPPictureRescale, WebPValidateConfig,
};

use crate::{Dimensions, InputOutput};

pub fn optimize<T>(config: &T) -> anyhow::Result<()>
where
    T: InputOutput + Dimensions,
{
    let input_image = ImageReader::open(config.input_path())?
        .with_guessed_format()?
        .decode()?;

    let dimensions = input_image.dimensions();
    let dimension_width = i32::try_from(dimensions.0)?;
    let dimension_height = i32::try_from(dimensions.1)?;
    let rgba_image = input_image.into_rgba8();

    let mut webp_config = WebPConfig::new().map_err(|()| anyhow!("Error creating WebP config"))?;
    webp_config.lossless = 1;
    webp_config.alpha_compression = 1;
    webp_config.quality = 1.0;

    let mut picture = WebPPicture::new().map_err(|()| anyhow!("Error creating WebP picture"))?;
    picture.use_argb = 1;
    picture.width = dimension_width;
    picture.height = dimension_height;

    let mut ww: ::core::mem::MaybeUninit<WebPMemoryWriter> = ::core::mem::MaybeUninit::uninit();
    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr().cast::<std::ffi::c_void>();

    unsafe {
        if WebPValidateConfig(&webp_config) == 0 {
            bail!("Invalid WebP configuration");
        }

        WebPMemoryWriterInit(ww.as_mut_ptr());
        WebPPictureImportRGBA(&mut picture, rgba_image.as_ptr(), dimension_width * 4);

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
            bail!("Error encoding WebP: {:?}", picture.error_code);
        }

        let ww = ww.assume_init();
        let contents = std::slice::from_raw_parts(ww.mem, ww.size);

        std::fs::write(config.output_path(), contents)?;

        WebPPictureFree(&mut picture);
    }

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
