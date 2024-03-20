use image_resizer::utils::gifsicle;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        disable_help_flag = true,
        after_help = "
Examples:

  image-resizer convert --width 100 --height 100 input.jpg output.jpg

"
    )]
    /// Convert images from one format to another
    Convert {
        #[clap(long, action = clap::ArgAction::HelpLong)]
        help: Option<bool>,
        /// Input image path
        input_path: String,
        /// Output image path
        output_path: String,
        /// Width of the output image
        /// If not set, the width will be the same as the input image
        #[clap(short, long)]
        width: Option<u32>,
        /// Height of the output image
        /// If not set, the height will be the same as the input image
        #[clap(short, long)]
        height: Option<u32>,
    },
}

fn main() {
    let cli = Cli::parse();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
