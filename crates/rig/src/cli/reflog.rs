use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::RefLog {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::RefLog::run() is not implemented yet")
    }
}
