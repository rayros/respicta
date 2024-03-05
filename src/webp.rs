use image::GenericImageView;
use libwebp_sys::{
    VP8StatusCode, WebPConfig, WebPEncode, WebPEncodingError, WebPMemoryWrite,
    WebPMemoryWriterInit, WebPPicture, WebPPictureImportRGBA, WebPPictureRescale,
    WebPValidateConfig,
};

pub fn optimize(
    input_path: &str,
    output_path: &str,
    target_width: i32,
    target_height: i32,
) -> std::result::Result<(), libwebp_sys::WebPEncodingError> {
    let input_image = image::open(input_path).expect("Failed to open input image");
    let dimensions = input_image.dimensions();
    let rgba_image = input_image.into_rgba8();

    let mut config = WebPConfig::new().unwrap();
    config.lossless = 0;
    config.alpha_compression = 1;
    config.quality = 1.0;

    let mut picture = WebPPicture::new().unwrap();
    picture.use_argb = 1;
    picture.width = dimensions.0 as i32;
    picture.height = dimensions.1 as i32;

    let mut ww = std::mem::MaybeUninit::uninit();
    picture.writer = Some(WebPMemoryWrite);
    picture.custom_ptr = ww.as_mut_ptr() as *mut std::ffi::c_void;

    unsafe {
        if WebPValidateConfig(&config) == 0 {
            return Err(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        }
        WebPMemoryWriterInit(ww.as_mut_ptr());
        WebPPictureImportRGBA(
            &mut picture,
            rgba_image.as_ptr(),
            i32::try_from(dimensions.0).unwrap() * 4,
        );
        WebPPictureRescale(&mut picture, target_width, target_height);
        let encode_result = WebPEncode(&config, &mut picture);
        let ww = ww.assume_init();
        if encode_result == VP8StatusCode::VP8_STATUS_OK as i32 {
            return Err(picture.error_code);
        }

        let contents = std::slice::from_raw_parts(ww.mem, ww.size);
        std::fs::write(output_path, contents).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn webp_optimize() -> Result<(), libwebp_sys::WebPEncodingError> {
        use super::*;
        let input_png_path = "tests/files/issue-159.png";

        let output_webp_path = "target/issue-159.webp";

        optimize(input_png_path, output_webp_path, 100, 0)
    }
}
