//! rig の CLI 定義とサブコマンドの実行。
//!
//! バイナリ (`src/main.rs`) はこのクレートを呼ぶだけの薄いラッパー。
//! サブコマンドの実装が進んだら、ロジックをここから公開してテストから
//! 使えるようにする。

pub mod cli;
pub mod run;

use crate::cli::command::{Cli, Main, SubCommand};
use crate::run::Run;

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
