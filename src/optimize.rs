use crate::core::gif2webp;
pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
}

fn resolve_extension(path: &str) -> &str {
    let extension = path.split('.').last().unwrap();
    extension
}

pub fn optimize(config: &Config) {
    let input_extension = resolve_extension(config.input_path);
    let output_extension = resolve_extension(config.output_path);
    let input_path = config.input_path;
    let output_path = config.output_path;
    let width = config.width;
    match (input_extension, output_extension) {
        ("gif", "webp") => gif2webp::optimize(&gif2webp::Config {
            input_path,
            output_path,
            width,
        }),
        // ("png", "webp") => webp::optimize(config.input_path, config.output_path, 100, 0).unwrap(),
        (_, _) => {
            // Handle remaining cases here
        }
    }
}
