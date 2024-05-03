#[cfg(feature = "cli")]
mod cli {
    use axum::Router;
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;
    use tokio::{net::TcpListener, signal};

    #[derive(Parser)]
    #[command(version, about, long_about = None, arg_required_else_help = true)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        #[clap(
            arg_required_else_help = true,
            disable_help_flag = true,
            after_help = "\
                Examples: \n\
                \n\
                respicta convert --width 100 --height 100 input.jpg output.jpg
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
        /// Start a server
        Server {
            /// Address to bind the server (default: 0.0.0.0:3000)
            #[clap(short, long)]
            address: Option<String>,
            /// Maximum file size in bytes (default: 10MB)
            #[clap(short, long)]
            limit: Option<usize>,
        },
        /// Start a command server
        CommandServer {
            /// Address to bind the server (default: 0.0.0.0:3000)
            #[clap(short, long)]
            address: Option<String>,
        },
    }

    pub async fn start_server(address: Option<String>, service: Router) -> std::io::Result<()> {
        let address = address.unwrap_or_else(|| "0.0.0.0:3000".to_string());
        let listener = TcpListener::bind(address.clone()).await;
        match listener {
            Ok(listener) => {
                let version = option_env!("CARGO_PKG_VERSION");
                if let Some(version) = version {
                    println!("Respicta v{version}");
                }
                println!("Server started at http://{address}");
                axum::serve(listener, service)
                    .with_graceful_shutdown(shutdown_signal())
                    .await
            }
            Err(error) => Err(error),
        }
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
    use crate::cli::{start_server, Cli, Commands};
    use clap::Parser;
    use respicta::{command_server, convert, server, Config};

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Convert {
            input_path,
            output_path,
            width,
            height,
            ..
        }) => convert(&Config::new(input_path, output_path, width, height)).unwrap(),
        Some(Commands::Server { address, limit }) => {
            start_server(address, server::app(limit)).await.unwrap();
        }
        Some(Commands::CommandServer { address }) => {
            start_server(address, command_server::app()).await.unwrap();
        }
        None => unreachable!(),
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    unimplemented!("Please enable the `cli` feature to use the CLI")
}
