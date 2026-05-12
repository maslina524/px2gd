use clap::{Parser, Subcommand};

use crate::io::IOFlags;

mod io;

#[derive(Parser)]
#[command(name = "px2gd")]
#[command(about = "A convenient CLI tool for generating raster images in Geometry Dash", long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = false)]
struct Cli {
    #[arg(short = 'f', long = "file")]
    file: String,

    #[arg(short = 's', long = "stdout")]
    stdout: bool, // Outputs everything to stdout

    #[arg(short = 'j', long = "json")]
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

    let ioflags = IOFlags {
        stdout: cli.stdout,
        json: cli.json,
        only_result: cli.only_result
    };
    
    let cmd = match cli.command {
        Some(c) => {}
        None => {}
    };
}
