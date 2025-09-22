mod cli;
mod commands;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::*;
mod config;

fn main() -> Result<()> {
   let cli = Cli::parse();

   match cli.command {
      Some(Commands::Completions { shell }) => {
         completions::run(shell)?;
      }
      None => launch::run(cli.url)?,
   }
   Ok(())
}
