use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Grep {
    fn run(&self) -> Result<()> {
        todo!("Implement command::Grep::run()")
    }
}
