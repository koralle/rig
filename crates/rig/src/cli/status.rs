use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Status {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Status::run() is not implemented yet")
    }
}
