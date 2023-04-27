use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Log {
    fn run(&self) -> Result<()> {
        todo!("Implement command::Log::run()")
    }
}
