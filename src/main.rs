mod cli;
mod request;
mod run;

use anyhow::Result;
use cli::command::SubCommand;

use clap::Parser;

use std::process;

use crate::cli::command::Cli;
use crate::run::Run;

use self::cli::command::Main;

impl Run for Cli {
    fn run(&self) -> anyhow::Result<()> {
        let subcmd = self.subcmd.as_ref().unwrap_or(&SubCommand::Main(Main {}));

        match subcmd {
            SubCommand::Main(cmd) => cmd.run(),
            SubCommand::Log(cmd) => cmd.run(),
            SubCommand::Show(cmd) => cmd.run(),
            SubCommand::RefLog(cmd) => cmd.run(),
            SubCommand::Blame(cmd) => cmd.run(),
            SubCommand::Grep(cmd) => cmd.run(),
            SubCommand::Refs(cmd) => cmd.run(),
            SubCommand::Stash(cmd) => cmd.run(),
            SubCommand::Status(cmd) => cmd.run(),
        }
    }
}

fn main() -> Result<()> {
    match Cli::parse().run() {
        Ok(_) => {
            // TODO: handle exit code;
            process::exit(0);
        }
        Err(err) => {
            // TODO: handle exit code;
            eprintln!("[rig error]: {:#}", err);
            process::exit(1);
        }
    };
}
