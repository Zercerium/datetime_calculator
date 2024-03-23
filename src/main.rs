use args::{CalcArg, ConfigArg};
use clap::Parser;
use std::ops::Add;

use crate::{
    args::Args, config::Config, duration_parser::Duration, input_parser::PrimitiveDateTime,
};

mod args;
mod config;
mod duration_parser;
mod input_parser;

fn main() {
    let args = Args::parse();

    let cmd = args.command.unwrap_or(args::Command::Calc(args.calc_arg));

    match cmd {
        args::Command::Calc(v) => calc(v),
        args::Command::Config(v) => config(v),
    }
}

fn calc(v: CalcArg) {
    let settings = Config::new().unwrap();

    let input0 = v.start.unwrap().parse::<PrimitiveDateTime>().unwrap();
    let input1 = v.duration_or_end.unwrap().parse::<Duration>().unwrap();

    let result = input0.0.add(input1.0);

    let format = settings.formats.get("default").unwrap();
    let format = time::format_description::parse(format).unwrap();
    let result = result.format(&format).unwrap();

    println!("{}", result);
}

fn config(v: ConfigArg) {
    if v.create_config {
        Config::create_default_config_file().unwrap();
    }
}
