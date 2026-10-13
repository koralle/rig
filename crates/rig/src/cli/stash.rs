use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Stash {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Stash::run() is not implemented yet")
    }
}
