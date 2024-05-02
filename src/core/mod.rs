use std::path::PathBuf;

use crate::PathAccessor;

pub mod gif2gif;
pub mod gif2webp;
pub mod jpeg2jpeg;
pub mod jpeg2webp;
pub mod png2jpeg;
pub mod png2png;
pub mod png2webp;
pub mod webp2webp;

pub struct PathIO<'a> {
    input_path: &'a PathBuf,
    output_path: &'a PathBuf,
}

impl<'a> PathIO<'a> {
    #[must_use]
    pub fn new(input_path: &'a PathBuf, output_path: &'a PathBuf) -> Self {
        Self {
            input_path,
            output_path,
        }
    }
}

impl<'a> PathAccessor for PathIO<'a> {
    fn input_path(&self) -> &PathBuf {
        self.input_path
    }

    fn output_path(&self) -> &PathBuf {
        self.output_path
    }
}
