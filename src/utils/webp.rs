use anyhow::bail;
use image::GenericImageView;
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPMemoryWrite, WebPMemoryWriter, WebPMemoryWriterInit,
    WebPPicture, WebPPictureFree, WebPPictureImportRGBA, WebPPictureRescale, WebPValidateConfig,
};

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

pub fn optimize(config: &Config) -> anyhow::Result<()> {
    println!(
        "WEBP: Optimizing image file: {} -> {}",
        config.input_path, config.output_path
    );

    let input_image = image::open(config.input_path)?;

    let dimensions = input_image.dimensions();
    let rgba_image = input_image.into_rgba8();

    let mut webp_config = WebPConfig::new().unwrap();
    webp_config.lossless = 0;
    webp_config.alpha_compression = 1;
    webp_config.quality = 1.0;

    let mut picture = WebPPicture::new().unwrap();
    picture.use_argb = 1;
    picture.width = dimensions.0 as i32;
    picture.height = dimensions.1 as i32;

    let mut ww: ::core::mem::MaybeUninit<WebPMemoryWriter> = ::core::mem::MaybeUninit::uninit();
    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr().cast::<std::ffi::c_void>();

    unsafe {
        if WebPValidateConfig(&webp_config) == 0 {
            bail!("Invalid WebP configuration");
        }
        WebPMemoryWriterInit(ww.as_mut_ptr());
        WebPPictureImportRGBA(
            &mut picture,
            rgba_image.as_ptr(),
            i32::try_from(dimensions.0).unwrap() * 4,
        );
        let target_width = config.width.unwrap_or(0);
        let target_height = config.height.unwrap_or(0);
        WebPPictureRescale(&mut picture, target_width, target_height);
        let encode_result = WebPEncode(&webp_config, &mut picture);
        if encode_result == VP8StatusCode::VP8_STATUS_OK as i32 {
            bail!("Error encoding WebP: {:?}", picture.error_code);
        }
        let ww = ww.assume_init();
        let contents = std::slice::from_raw_parts(ww.mem, ww.size);
        std::fs::write(config.output_path, contents).unwrap();

        WebPPictureFree(&mut picture);
    }

    Ok(())
}

pub struct GifConfig<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
}

pub fn optimize_gif(config: &GifConfig) -> std::result::Result<(), std::io::Error> {
    println!(
        "WEBP: Optimizing gif file: {} -> {}",
        config.input_path, config.output_path
    );
    let input_path = config.input_path;
    let output_path = config.output_path;
    let command = format!("gif2webp -o {output_path} -q 75 -m 6 -mt -v {input_path}",);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let code = output.status.code();

    match code {
        Some(0) => Ok(()),
        Some(_) | None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "gif2webp failed",
        )),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn webp_optimize_png_to_webp() {
        use super::*;

        optimize(&Config {
            input_path: "tests/files/issue-159.png",
            output_path: "target/issue-159.webp",
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_static() {
        use super::*;

        optimize(&Config {
            input_path: "tests/files/test1.gif",
            output_path: "target/gif_test1_static.webp",
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    fn webp_optimize_gif_to_webp_2() {
        use super::*;

        optimize_gif(&GifConfig {
            input_path: "tests/files/test1.gif",
            output_path: "target/webp_gif_test1.webp",
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "gif2webp failed"]
    fn webp_optimize_gif_to_webp_panic() {
        use super::*;

        optimize_gif(&GifConfig {
            input_path: "tests/files/not_existing.gif",
            output_path: "target/webp_gif_test1.webp",
        })
        .unwrap();
    }
}
