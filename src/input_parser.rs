use std::str::FromStr;

use nom::{
    combinator::{map_res, rest},
    error::Error,
    Finish, IResult,
};
use time::macros::format_description;

const CMD_TODAY: &[&str] = &["today", "t"];

enum Commands {
    TODAY,
}

impl Commands {}

pub struct PrimitiveDateTime(pub time::PrimitiveDateTime);

impl FromStr for PrimitiveDateTime {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_options(s).finish() {
            Ok((_remaining, date_time)) => Ok(Self(date_time)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_options(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    parse_date_time_dmy_dot(input)
}

fn parse_date_time_dmy_dot(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    let format = format_description!("[day].[month].[year]");
    let (_, date_time) = map_res(rest, |rest| time::Date::parse(rest, format))(input)?;
    Ok((input, date_time.midnight()))
}
