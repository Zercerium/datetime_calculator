use std::str::FromStr;

use nom::{
    branch::alt,
    combinator::{map_res, rest},
    error::Error,
    Finish, IResult,
};
use time::ext::NumericalDuration;
use time::macros::format_description;

const CMD_TODAY: &str = "today";
const CMD_T: &str = "t";
const CMD_TOMORROW: &str = "tomorrow";
const CMD_TM: &str = "tm";

enum Command {
    TODAY,
    TOMORROW,
}

impl Command {
    fn to_date_time(&self) -> time::PrimitiveDateTime {
        match self {
            Self::TODAY => {
                let now = time::OffsetDateTime::now_local().unwrap();
                time::PrimitiveDateTime::new(now.date(), now.time())
            }
            Self::TOMORROW => {
                let now = time::OffsetDateTime::now_local().unwrap();
                time::PrimitiveDateTime::new(now.date(), now.time()) + 1.days()
            }
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CMD_T | CMD_TODAY => Ok(Self::TODAY),
            CMD_TM | CMD_TOMORROW => Ok(Self::TOMORROW),
            _ => Err("Invalid command".to_string()),
        }
    }
}

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
    alt((parse_date_time_dmy_dot, parse_commands))(input)
}

fn parse_date_time_dmy_dot(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    let format = format_description!("[day padding:none].[month padding:none].[year]");
    let (_, date_time) = map_res(rest, |rest| time::Date::parse(rest, format))(input)?;
    Ok((input, date_time.midnight()))
}

fn parse_commands(input: &str) -> IResult<&str, time::PrimitiveDateTime> {
    let (_, command) = map_res(rest, Command::from_str)(input)?;
    Ok((input, command.to_date_time()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_duration() {}

    #[test]
    fn test_parse_duration1() {
        let date_1 = time::macros::datetime!(2021-01-01 00:00:00);
        let date_2 = time::macros::datetime!(2021-01-02 00:00:00);

        let ex_duration = time::Duration::days(1);
        let ex_neg_duration = -ex_duration;

        let res_duration = date_2 - date_1;
        let res_neg_duration = date_1 - date_2;

        assert_eq!(ex_duration, res_duration);
        assert_eq!(ex_neg_duration, res_neg_duration);
    }
}
