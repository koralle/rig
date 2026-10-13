use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Log {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Log::run() is not implemented yet")
    }
}
