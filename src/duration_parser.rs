use std::str::FromStr;

use nom::{
    character::complete::{anychar, digit1},
    combinator::{eof, map_res, opt},
    error::Error,
    multi::many1,
    Finish, IResult,
};

pub struct Duration(pub time::Duration);

impl From<Duration> for time::Duration {
    fn from(duration: Duration) -> Self {
        duration.0
    }
}

impl FromStr for Duration {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_durations(s).finish() {
            Ok((_remaining, duration)) => Ok(Self(duration)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_durations(input: &str) -> IResult<&str, time::Duration> {
    // seconds, minutes, hours, days, weeks
    let (input, sign) = parse_optional_sign(input)?;
    let (input, durations) = many1(parse_single_duration)(input)?;
    eof(input)?;
    let duration = durations.iter().sum();
    let duration = sign.apply(duration);
    Ok((input, duration))
}

fn parse_optional_sign(input: &str) -> IResult<&str, Sign> {
    let sign = map_res(anychar, Sign::try_from);
    let (input, sign) = opt(sign)(input)?;
    let sign = sign.unwrap_or_default();
    Ok((input, sign))
}

#[derive(Default, Debug)]
enum Sign {
    #[default]
    Positive,
    Negative,
}

impl Sign {
    fn apply(&self, duration: time::Duration) -> time::Duration {
        match self {
            Sign::Positive => duration,
            Sign::Negative => -duration,
        }
    }
}

const CHAR_PLUS: char = '+';
const CHAR_MINUS: char = '-';

impl TryFrom<char> for Sign {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            CHAR_PLUS => Ok(Self::Positive),
            CHAR_MINUS => Ok(Self::Negative),
            char => Err(format!("Invalid sign: `{}`", char)),
        }
    }
}

impl From<Sign> for char {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Positive => CHAR_PLUS,
            Sign::Negative => CHAR_MINUS,
        }
    }
}

fn parse_single_duration(input: &str) -> IResult<&str, time::Duration> {
    let (input, duration) = map_res(digit1, str::parse)(input)?;
    let (input, unit) = map_res(anychar, TimeUnit::try_from)(input)?;
    let duration = unit.to_duration(duration);
    Ok((input, duration))
}

const CHAR_SECOND: char = 's';
const CHAR_MINUET: char = 'm';
const CHAR_HOUR: char = 'h';
const CHAR_DAY: char = 'd';
const CHAR_WEEK: char = 'w';

#[derive(Debug)]
enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl TryFrom<char> for TimeUnit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            CHAR_SECOND => Ok(Self::Second),
            CHAR_MINUET => Ok(Self::Minute),
            CHAR_HOUR => Ok(Self::Hour),
            CHAR_DAY => Ok(Self::Day),
            CHAR_WEEK => Ok(Self::Week),
            char => Err(format!("Invalid time unit: `{}`", char)),
        }
    }
}

impl From<TimeUnit> for char {
    fn from(unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Second => CHAR_SECOND,
            TimeUnit::Minute => CHAR_MINUET,
            TimeUnit::Hour => CHAR_HOUR,
            TimeUnit::Day => CHAR_DAY,
            TimeUnit::Week => CHAR_WEEK,
        }
    }
}

impl TimeUnit {
    fn to_duration(&self, amount: i64) -> time::Duration {
        let f = match self {
            TimeUnit::Second => time::Duration::seconds,
            TimeUnit::Minute => time::Duration::minutes,
            TimeUnit::Hour => time::Duration::hours,
            TimeUnit::Day => time::Duration::days,
            TimeUnit::Week => time::Duration::weeks,
        };
        f(amount)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_duration() {
        let input = "-10s".to_string();
        let expected = time::Duration::seconds(-10);
        let actual = input.parse::<Duration>().unwrap().0;
        assert_eq!(expected, actual);
    }
}
