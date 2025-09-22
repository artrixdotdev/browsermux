use crate::cli::Cli;
use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use clap_complete_nushell::Nushell;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CompletionShell {
   Bash,
   Elvish,
   Fish,
   PowerShell,
   Zsh,
   Nushell,
}

impl From<CompletionShell> for Shell {
   fn from(shell: CompletionShell) -> Self {
      match shell {
         CompletionShell::Bash => Shell::Bash,
         CompletionShell::Elvish => Shell::Elvish,
         CompletionShell::Fish => Shell::Fish,
         CompletionShell::PowerShell => Shell::PowerShell,
         CompletionShell::Zsh => Shell::Zsh,
         CompletionShell::Nushell => unreachable!("Handled before it gets here"),
      }
   }
}

pub fn run(shell: CompletionShell) -> Result<()> {
   let mut out = std::io::stdout();
   let name = env!("CARGO_PKG_NAME");
   let mut command = Cli::command();

   match shell {
      CompletionShell::Nushell => generate(Nushell, &mut command, name, &mut out),

      other => generate::<Shell, _>(other.into(), &mut command, name, &mut out),
   }

   Ok(())
}
