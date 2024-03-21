use std::path::PathBuf;

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Config<'_> {
    pub fn to_args(&self) -> String {
        let output_path = self.output_path.display();
        let input_path = self.input_path.display();
        let mut result = format!("-O3 --output {output_path}");
        if let Some(width) = self.width {
            result = format!("{result} --resize-width {width}");
        }
        if let Some(height) = self.height {
            result = format!("{result} --resize-height {height}");
        }
        result = format!("{result} {input_path}");
        result
    }
}

pub fn optimize(config: Config) -> std::result::Result<(), std::io::Error> {
    let args = config.to_args();
    let command = format!("gifsicle {args}");
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let code = output.status.code();

    match code {
        Some(0) => Ok(()),
        Some(_) | None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "gifsicle failed",
        )),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn gifsicle() {
        use super::*;

        optimize(Config {
            input_path: &PathBuf::from("tests/files/gifsicle_test1.gif"),
            output_path: &PathBuf::from("target/gifsicle_test1.gif"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    fn gifsicle_without_width_and_height() {
        use super::*;

        optimize(Config {
            input_path: &PathBuf::from("tests/files/gifsicle_test1.gif"),
            output_path: &PathBuf::from("target/gifsicle_without_width_and_height_test2.gif"),
            width: None,
            height: None,
        })
        .unwrap();
    }
}
