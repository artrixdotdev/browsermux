mod cli;
mod commands;
use std::fs;

use anyhow::{Result, ensure};
use clap::Parser;
use cli::{Cli, Commands};
use commands::*;
use config::{Config, File as ConfigFile};

use settings::*;
mod settings;

fn main() -> Result<()> {
   let cli = Cli::parse();
   let config_path = fs::canonicalize(cli.config.unwrap_or("~/browsermux/config.toml".into()))?;

   ensure!(config_path.exists(), "config file not found");
   let config_path = config_path.to_string_lossy().to_string();

   let config = Config::builder()
      .add_source(ConfigFile::with_name(&config_path))
      .build()?;

   let settings = config.try_deserialize::<Settings>()?;

   match cli.command {
      Some(Commands::Completions { shell }) => {
         completions::run(shell)?;
      }
      None => launch::run(settings, cli.url)?,
   }
   Ok(())
}
