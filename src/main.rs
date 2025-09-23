mod cli;
mod commands;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use cli::{Cli, Commands};
use commands::*;
use config::{Config, File as ConfigFile};

use settings::*;
mod settings;
mod util;

fn main() -> Result<()> {
   let cli = Cli::parse();
   let config_path = util::expand_dir(
      cli.config
         .unwrap_or("~/.config/browsermux/config.toml".into()),
   );

   println!("config path: {config_path:?}");

   ensure!(config_path.exists(), "config file not found");
   let config_path = config_path.to_string_lossy().to_string();

   let config = Config::builder()
      .add_source(ConfigFile::with_name(&config_path))
      .build()
      .with_context(|| format!("failed to load config file at {config_path}"))?;

   let settings = config.try_deserialize::<Settings>()?;

   match cli.command {
      Some(Commands::Completions { shell }) => {
         completions::run(shell)?;
      }
      None => launch::run(settings, cli.url)?,
   }
   Ok(())
}
