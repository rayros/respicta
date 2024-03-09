use std::ffi::CString;

pub struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub width: Option<i32>,
}

impl Config<'_> {
    pub fn to_args(&self) -> Vec<CString> {
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
        .map(|arg| CString::new(arg).unwrap())
        .collect();
        args
    }
}

pub fn optimize(config: &Config) {
    let args: Vec<CString> = config.to_args();
    let gifsicle_argv: Vec<*const i8> = args.iter().map(|a| a.as_ptr()).collect();
    let gifsicle_argv_len = i32::try_from(gifsicle_argv.len()).unwrap();
    unsafe {
        gifsicle::gifsicle_main(gifsicle_argv_len, gifsicle_argv.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gifsicle() {
        use super::*;
        // Specify the input PNG file path
        let input_path = "tests/files/test1.gif";

        // Specify the output PNG file path (optional)
        let output_path = "target/gifsicle_test1.gif";

        optimize(&Config {
            input_path,
            output_path,
            width: Some(100),
        });
    }
}
