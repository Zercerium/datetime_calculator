use clap::{command, Parser};
use std::ops::Add;

use crate::{duration_parser::Duration, input_parser::PrimitiveDateTime};

mod duration_parser;
mod input_parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input
    input0: String,
    input1: String,
}

fn main() {
    let args = Args::parse();

    let input0 = args.input0.parse::<PrimitiveDateTime>().unwrap();
    let input1 = args.input1.parse::<Duration>().unwrap();

    let result = input0.0.add(input1.0);

    println!("{:?}", result);
}
