use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Refs {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Refs::run() is not implemented yet")
    }
}
