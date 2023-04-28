use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Stash {
    fn run(&self) -> Result<()> {
        todo!("Implement command::Stash::run()")
    }
}
