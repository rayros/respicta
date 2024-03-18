use std::str::FromStr;
pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

impl Config<'_> {
    pub fn to_args(&self) -> Vec<String> {
        println!("To args: {:?}", self.width);
        let input = vec![self.input_path];
        let optional_width = if let Some(width) = self.width {
            vec!["--resize-width".to_string(), width.to_string()]
        } else {
            vec![]
        };
        let optional_height = if let Some(height) = self.height {
            vec!["--resize-height".to_string(), height.to_string()]
        } else {
            vec![]
        };
        let optional_width_vec_str: Vec<&str> = optional_width
            .iter()
            .map(std::string::String::as_str)
            .collect();
        let optional_height_vec_str: Vec<&str> = optional_height
            .iter()
            .map(std::string::String::as_str)
            .collect();
        let args: Vec<_> = [
            vec!["", "-O3", "--output", self.output_path],
            optional_width_vec_str,
            optional_height_vec_str,
            input,
        ]
        .concat()
        .into_iter()
        .map(|arg| String::from_str(arg).unwrap())
        .collect();
        args
    }
}

pub fn optimize(config: Config) -> std::result::Result<(), std::io::Error> {
    let args = config.to_args().join(" ");
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
            input_path: "tests/files/gifsicle_test1.gif",
            output_path: "target/gifsicle_test1.gif",
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
