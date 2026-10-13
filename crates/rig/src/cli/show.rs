use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Show {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Show::run() is not implemented yet")
    }
}
