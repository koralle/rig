use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Blame {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Blame::run() is not implemented yet")
    }
}
