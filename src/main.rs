use std::process;

use clap::{Error, Parser, error::ErrorKind};
use temp_convert::{
    Args,
    utils::{COLOR_ERROR, COLOR_INFO, COLOR_RESET},
};

fn main() {
    let args = Args::try_parse().unwrap_or_else(|error: Error| match error.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
            println!("\n{}{}{}", COLOR_INFO, error.to_string(), COLOR_RESET);
            process::exit(0);
        }
        _ => {
            eprintln!("\n{}{}{}", COLOR_ERROR, error, COLOR_RESET);
            process::exit(1);
        }
    });

    match args.run() {
        Ok(output) => println!("\n{}", output),
        Err(error) => {
            eprintln!("\n{}{}{}", COLOR_ERROR, error, COLOR_RESET);
            process::exit(1);
        }
    }
}
