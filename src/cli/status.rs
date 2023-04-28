use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Status {
    fn run(&self) -> Result<()> {
        todo!("Implement command::Status::run()")
    }
}
