pub mod gifsicle;
pub mod magick;
pub mod oxipng;
pub mod webp;

#[must_use]
pub fn fit(width: u32, height: u32, max_width: u32, max_height: u32) -> (u32, u32) {
    let width_ratio = f64::from(max_width) / f64::from(width);
    let height_ratio = f64::from(max_height) / f64::from(height);

    if width_ratio < height_ratio {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let new_height = (f64::from(height) * width_ratio) as u32;
        (max_width, new_height)
    } else {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let new_width = (f64::from(width) * height_ratio) as u32;
        (new_width, max_height)
    }
}
