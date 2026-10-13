use anyhow::Result;

use crate::cli::command;

use crate::run::Run;

impl Run for command::Main {
    fn run(&self) -> Result<()> {
        anyhow::bail!("command::Main::run() is not implemented yet")
    }
}
