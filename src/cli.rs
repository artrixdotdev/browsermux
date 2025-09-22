use std::path::PathBuf;

use clap::{Parser, Subcommand};
use url::Url;

use crate::commands::completions::CompletionShell;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
   /// The URL to open
   pub url: Url,

   /// Sets a custom config file
   #[arg(short, long, value_name = "FILE")]
   pub config: Option<PathBuf>,

   #[command(subcommand)]
   pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
   /// Generates completions for a shell and writes them to stdout
   Completions {
      /// Shell to generate completions for
      #[arg(value_enum)]
      shell: CompletionShell,
   },
}
