use anyhow::Result;
use clap::Parser;
use std::process;

use rig::cli::command::Cli;
use rig::run::Run;

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
