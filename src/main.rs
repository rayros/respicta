use std::ffi::CString;

mod magick;
mod oxipng;
mod webp;
use oxipng::optimize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    magick::resize_and_auto_orient(
        "tests/files/img20230418182427.jpg",
        "target/magick_out_image_1.jpg",
        240,
        100,
    )?;
    let args: Vec<_> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let argv: Vec<_> = args.iter().map(|a| a.as_ptr()).collect();

    unsafe {
        gifsicle::gifsicle_main(argv.len() as _, argv.as_ptr());
    }

    // Specify the input PNG file path
    // optimize(input_png_path, output_png_path)

    Ok(())
}
