use std::{error::Error, fs::File};

use mozjpeg::ScanMode;
use std::io::Write;

fn optimize(input_path: &str, output_path: &str, quality: f32) -> Result<(), Box<dyn Error>> {
    let d = mozjpeg::Decompress::with_markers(mozjpeg::NO_MARKERS).from_path(input_path)?;

    // d.width(); // FYI
    // d.height();
    // d.color_space() == mozjpeg::ColorSpace::JCS_YCbCr;
    // for marker in d.markers() { /* read metadata or color profiles */ }

    // rgb() enables conversion
    let mut image = d.rgb()?;
    // image.color_space() == mozjpeg::ColorSpace::JCS_RGB;

    let pixels: Vec<u8> = image.read_scanlines()?;
    let width = image.width();
    let height = image.height();
    image.finish()?;
    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    comp.set_scan_optimization_mode(ScanMode::Auto);
    comp.set_quality(quality);
    comp.set_progressive_mode();
    comp.set_optimize_coding(true);
    comp.set_optimize_scans(true);
    comp.set_size(width, height);
    comp.set_chroma_sampling_pixel_sizes((2, 2), (2, 2));
    let mut comp = comp.start_compress(Vec::new())?; // any io::Write will work

    // replace with your image data
    comp.write_scanlines(&pixels[..])?;

    let writer = comp.finish()?;

    let mut output_file = File::create(output_path)?;
    output_file.write_all(&writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    fn mozjpeg_optimize() -> Result<(), Box<dyn Error>> {
        use super::*;
        let input_path = "tests/files/orientation_test.jpg";

        let output_path = "target/mozjpeg_orientation_test.jpg";

        optimize(input_path, output_path, 75.0)
    }
}
