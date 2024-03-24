use clap::{Parser, Subcommand};
use image_resizer::{convert, server};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        arg_required_else_help = true,
        disable_help_flag = true,
        after_help = "
Examples:

  image-resizer convert --width 100 --height 100 input.jpg output.jpg

"
    )]
    /// Convert images from one format to another
    Convert {
        /// Input image path
        input_path: PathBuf,
        /// Output image path
        output_path: PathBuf,
        /// Width of the output image
        /// If not set, the width will be the same as the input image
        #[clap(short, long)]
        width: Option<u32>,
        /// Height of the output image
        /// If not set, the height will be the same as the input image
        #[clap(short, long)]
        height: Option<u32>,
        #[clap(long, action = clap::ArgAction::HelpLong)]
        help: Option<bool>,
    },
    /// Server for the image resizer
    Server {
        /// Address to bind the server to (default: 0.0.0.0:3000)
        address: Option<String>,
        /// Maximum file size in bytes (default: 10MB)
        limit: Option<usize>,
    },
}

#[tokio::main]
async fn main() {
    use image_resizer::Config;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Convert {
            input_path,
            output_path,
            width,
            height,
            ..
        }) => {
            convert(&Config {
                input_path: &input_path,
                output_path: &output_path,
                width,
                height,
            })
            .unwrap();
        }
        Some(Commands::Server { address, limit }) => server::run(address, limit).await.unwrap(),
        None => unreachable!(),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
