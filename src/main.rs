use clap::{Parser, Subcommand};

mod io;

#[derive(Parser)]
#[command(name = "px2gd")]
#[command(about = "A convenient CLI tool for generating raster images in Geometry Dash", long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = false)]
struct Cli {
    #[arg(short = 'f', long = "file")]
    file: String,

    #[arg(short = 's')]
    stdout: bool, // Outputs everything to stdout

    #[arg(short = 'j')]
    json: bool, // Outputs everything to stdout

    #[arg(short = 'r', long = "only-result")]
    only_result: bool, // Outputs only the result of execution

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands { }
fn main() {
    let cli = Cli::parse();
    
    let cmd = match cli.command {
        Some(c) => {}
        None => {}
    };
}
