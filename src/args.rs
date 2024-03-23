use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub calc_arg: CalcArg,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Calc(CalcArg),
    Config(ConfigArg),
}

#[derive(clap::Args, Debug)]
pub struct CalcArg {
    /// start date
    #[arg(required = true)]
    pub start: Option<String>,
    /// Input
    #[arg(required = true)]
    pub duration_or_end: Option<String>,
    #[arg(short, long)]
    pub output_format: Option<String>,
    #[arg(short, long)]
    pub input_format: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct ConfigArg {
    #[arg(long)]
    pub create_config: bool,
}
