use crate::{Dimensions, InputOutput};

fn to_args<T>(config: &T) -> String
where
    T: InputOutput + Dimensions,
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

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Exit(i32),
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
    T: InputOutput + Dimensions,
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

        optimize(&Config {
            input_path: &"tests/files/gifsicle_test1.gif".into(),
            output_path: &"target/gifsicle_test1.gif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    fn gifsicle_without_width() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/gifsicle_test1.gif".into(),
            output_path: &"target/gifsicle_without_width_test2.gif".into(),
            width: None,
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    fn gifsicle_without_width_and_height() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/gifsicle_test1.gif".into(),
            output_path: &"target/gifsicle_without_width_and_height_test2.gif".into(),
            width: None,
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Exit(1)"]
    fn gifsicle_panic() {
        use super::*;

        optimize(&Config {
            input_path: &"tests/files/gifsicle_notexisting_test1.gif".into(),
            output_path: &"target/gifsicle_test1.gif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "Signal"]
    fn process_exit_code_terminated_by_signal_panic() {
        use super::*;

        process_exit_code(None).unwrap();
    }
}
