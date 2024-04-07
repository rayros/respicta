use std::path::PathBuf;

use oxipng::{Options, OutFile};

use crate::InputOutput;

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
}

impl InputOutput for Config<'_> {
    fn input_path(&self) -> &PathBuf {
        self.input_path
    }

    fn output_path(&self) -> &PathBuf {
        self.output_path
    }
}

/// # Errors
///
/// Returns an error if the optimization fails.
///
pub fn optimize<T>(config: &T) -> Result<(), oxipng::PngError>
where
    T: InputOutput,
{
    let input = &config.input_path().into();
    let output = &OutFile::from_path(config.output_path().clone());
    let options = &Options {
        strip: oxipng::StripChunks::Safe, // Optionally, strip metadata
        ..Options::default()
    };

    oxipng::optimize(input, output, options)
}

#[cfg(test)]
mod tests {
    #[test]
    fn oxipng_optimize() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/issue-159.png".into(),
            output_path: &"target/issue-159.png".into(),
        })
        .unwrap();
    }
}
