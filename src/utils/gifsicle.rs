use thiserror::Error;

use crate::{Dimensions, PathAccessor};

fn to_args<T>(config: &T) -> String
where
    T: PathAccessor + Dimensions,
{
    let output_path = config.output_path().display();
    let input_path = config.input_path().display();
    let mut result = format!("-O3 --output {output_path}");
    if let Some(width) = config.width() {
        result = format!("{result} --resize-width {width}");
    }
    if let Some(height) = config.height() {
        result = format!("{result} --resize-height {height}");
    }
    format!("{result} {input_path}")
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Io({0})")]
    Io(std::io::Error),
    #[error("Exit({0})")]
    Exit(i32),
    #[error("Signal")]
    Signal,
}

fn process_exit_code(code: Option<i32>) -> Result<(), Error> {
    match code {
        Some(0) => Ok(()),
        Some(code) => Err(Error::Exit(code)),
        None => Err(Error::Signal),
    }
}

/// # Errors
///
/// Returns an error if the gifsicle command fails.
///
pub fn optimize<T>(config: &T) -> Result<(), Error>
where
    T: PathAccessor + Dimensions,
{
    let args = to_args(config);
    let command = format!("gifsicle {args}");
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(Error::Io)?;

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
    fn gifsicle() {
        use super::*;

        optimize(&Config::new(
            "tests/files/gifsicle_test1.gif",
            "target/gifsicle_test1.gif",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    fn gifsicle_without_width() {
        use super::*;

        optimize(&Config::new(
            "tests/files/gifsicle_test1.gif",
            "target/gifsicle_without_width_test2.gif",
            None,
            Some(100),
        ))
        .unwrap();
    }

    #[test]
    fn gifsicle_without_width_and_height() {
        use super::*;

        optimize(&Config::new(
            "tests/files/gifsicle_test1.gif",
            "target/gifsicle_without_width_and_height_test2.gif",
            None,
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Exit(1)"]
    fn gifsicle_panic() {
        use super::*;

        optimize(&Config::new(
            "tests/files/gifsicle_notexisting_test1.gif",
            "target/gifsicle_test1.gif",
            Some(100),
            None,
        ))
        .unwrap();
    }

    #[test]
    #[should_panic = "Signal"]
    fn process_exit_code_terminated_by_signal_panic() {
        use super::*;

        process_exit_code(None).unwrap();
    }
}
