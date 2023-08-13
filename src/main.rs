#![allow(unused)]
use clap::Parser;

use crate::cli::Cli;

pub mod cli;
mod gui;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            cli::Cmd::Play(_) => todo!(),
            cli::Cmd::Record(_) => todo!(),
        },
        None => gui::run().unwrap(),
    }
}
