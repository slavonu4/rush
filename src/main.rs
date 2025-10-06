use anyhow::Context;
use clap::Parser;

use crate::cli::{CliArguments, Command};

mod cli;
mod commands;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = CliArguments::try_parse().context("Unable to parse CLI arguments")?;
    match args.command {
        Command::Decode { input } => commands::decode::invoke(input),
    }
}
