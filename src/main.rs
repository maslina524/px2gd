use clap::{Parser, Subcommand};
use std::path::Path;

use crate::io::IOFlags;

mod io;
mod generate;
mod object;

#[derive(Parser)]
#[command(name = "px2gd")]
#[command(about = "A convenient CLI tool for generating raster images in Geometry Dash", long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = false)]
struct Cli {
    #[arg(short = 'f', long = "file")]
    file: String,

    #[arg(short = 't', long = "target")]
    target: String,

    #[arg(short = 'x', long = "x-pos", default_value_t = 0.0)]
    x: f64,

    #[arg(short = 'y', long = "y-pos", default_value_t = 0.0)]
    y: f64,

    #[arg(short = 'S', long = "scale", default_value_t = 1.0)]
    scale: f32,

    #[arg(short = 's', long = "stdout")]
    stdout: bool, // Outputs everything to stdout

    #[arg(short = 'j', long = "json")]
    json: bool, // Formats the output as json

    #[arg(short = 'r', long = "only-result")]
    only_result: bool, // Outputs only the result of execution

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands { }

#[derive(Debug, PartialEq)]
enum Target {
    ObjectString,
    LiveEditor,
    GameSave
}

fn main() {
    let cli = Cli::parse();

    let ioflags = IOFlags {
        stdout: cli.stdout,
        json: cli.json,
        only_result: cli.only_result
    };

    let target = match cli.target.as_str() {
        "string" | "str" => Target::ObjectString,
        "live" | "ws" => Target::LiveEditor,
        "save" | "gamesave" => Target::GameSave,
        _ => {
            io::print_result(Result::<&str, &str>::Err("Invalid value in --target"), &ioflags);
            std::process::exit(1);
        }
    };
    
    let cmd = {
        let path = Path::new(&cli.file);
        generate::run(&path, cli.x, cli.y, cli.scale)
    };

    let ret: Result<String, String> = match target {
        Target::ObjectString => {
            cmd.map(
                |v| v.iter().map(
                    |s| s.to_string()
                ).collect::<String>()
            )
            .map_err(|e| e.to_string())
        },
        _ => {
            Err("Other options for --target are not yet available.".to_string())
        }
    };

    io::print_result(ret, &ioflags);

    
}
