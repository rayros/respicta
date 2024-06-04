use imgref::ImgExt;
use rgb::FromSlice;

use crate::{utils::fit, Dimensions, PathAccessor, Quality};
use resize::Type::Lanczos3;
use std::{fs::File, io::Write};

pub fn convert<T>(config: &T) -> std::result::Result<(), anyhow::Error>
where
    T: PathAccessor + Dimensions + Quality,
{
    let mut decoder = png::Decoder::new(File::open(config.input_path()).unwrap());
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("Color type: {:?}", reader.output_color_type());
    println!("Bit depth: {:?}", info.bit_depth);
    println!("Buffer size: {:?}", reader.output_buffer_size());
    println!("Output buffer size: {:?}", info.buffer_size());
    println!("Width: {:?}", info.width);
    println!("Height: {:?}", info.height);

    let bytes = &buf[..info.buffer_size()];
    let mut writer = File::create(config.output_path()).unwrap();
    let src = match reader.output_color_type() {
        (png::ColorType::Rgb, _) => {
            let mut rgba_bytes = Vec::with_capacity(info.width as usize * info.height as usize * 4);
            for chunk in bytes.chunks(3) {
                rgba_bytes.push(chunk[0]);
                rgba_bytes.push(chunk[1]);
                rgba_bytes.push(chunk[2]);
                rgba_bytes.push(255);
            }
            rgba_bytes
        }
        (png::ColorType::Rgba, _) => bytes.to_vec(),
        _ => {
            return Err(anyhow::anyhow!("Unsupported color type"));
        }
    };
    let width = config.width().unwrap_or(info.width);
    let height = config.height().unwrap_or(info.height);
    let (new_width, new_height) = fit(info.width, info.height, width, height);
    let mut dest = vec![0; (new_width * new_height * 4).try_into().unwrap()];
    let mut resizer = resize::new(
        info.width as usize,
        info.height as usize,
        new_width as usize,
        new_height as usize,
        resize::Pixel::RGBA8,
        Lanczos3,
    )?;
    resizer.resize(src.as_rgba(), dest.as_rgba_mut())?;

    let img = ravif::Img::new(dest.as_rgba(), new_width as usize, new_height as usize);
    let mut encoder = ravif::Encoder::new().with_speed(4);

    if let Some(quality) = config.quality() {
        encoder = encoder.with_quality(quality as f32);
    }

    let result = encoder.encode_rgba(img.as_ref()).unwrap();
    writer.write_all(&result.avif_file).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ConfigBuilder;

    #[test]
    fn png2avif() {
        use super::*;

        convert(
            &ConfigBuilder::default()
                .input_path("tests/files/png2avif_test1.png")
                .output_path("target/png2avif_test1.avif")
                .width(Some(100))
                .quality(Some(90))
                .build()
                .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn png2avif_2() {
        use super::*;

        convert(
            &ConfigBuilder::default()
                .input_path("tests/files/png2avif_test2.png")
                .output_path("target/png2avif_test2.avif")
                .width(Some(100))
                .quality(Some(90))
                .build()
                .unwrap(),
        )
        .unwrap();
    }
}
