use std::str::FromStr;
pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
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
        let optional_width_vec_str: Vec<&str> = optional_width
            .iter()
            .map(std::string::String::as_str)
            .collect();
        let args: Vec<_> = [
            vec!["", "-O3", "--output", self.output_path],
            optional_width_vec_str,
            input,
        ]
        .concat()
        .into_iter()
        .map(|arg| String::from_str(arg).unwrap())
        .collect();
        args
    }
}

pub fn optimize(config: Config) {
    println!(
        "GIFSICLE: Optimizing gif file: {} -> {}",
        config.input_path, config.output_path
    );
    let args = config.to_args().join(" ");
    let command = format!("gifsicle {args}");
    println!("Command: {:?}", command);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
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
        });
    }
}
