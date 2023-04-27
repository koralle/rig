use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Subcommand)]
pub enum SubCommand {
    #[clap(name = "", hide = true)]
    Main(Main),
    #[clap(name = "log")]
    Log(Log),
    #[clap(name = "show")]
    Show(Show),
    #[clap(name = "reflog")]
    RefLog(RefLog),
    #[clap(name = "blame")]
    Blame(Blame),
    #[clap(name = "grep")]
    Grep(Grep),
    #[clap(name = "refs")]
    Refs(Refs),
    #[clap(name = "stash")]
    Stash(Stash),
    #[clap(name = "status")]
    Status(Status),
}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Main {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Log {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Show {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct RefLog {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Blame {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Grep {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Refs {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Stash {}

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Status {}
