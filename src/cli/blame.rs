use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Blame {
    fn run(&self) -> Result<()> {
        todo!("Implement command::Blame::run()")
    }
}
