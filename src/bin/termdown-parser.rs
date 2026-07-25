#![allow(unused_variables)]
#![allow(dead_code)]

use chrono::Timelike;
use wks::prelude::*;

#[derive(Parser)]
struct Octopus {
    timestamps: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum Timestamp {
    Resolvable(Resolution),
    Unresolvable(String),
}

#[derive(Debug, PartialEq)]
enum Resolution {
    Later(String),
    Resolved(Container),
}

#[derive(Debug, PartialEq)]
struct Container(String, DateTime<Local>);

fn main() -> Result<()> {
    let Octopus { timestamps } = Octopus::parse();
    let now = Local::now();
    resolve(timestamps, now);
    Ok(())
}

fn resolve(arguments: Vec<String>, now: DateTime<Local>) -> Vec<Timestamp> {
    let mut first_pass = Vec::new();
    for arg in arguments.into_iter() {
        if arg.starts_with(['-', '+']) {
            first_pass.push(Timestamp::Resolvable(Resolution::Later(arg)));
        } else if arg.contains(':') {
            let resolved = resolve_colon(&arg, now);
            first_pass.push(Timestamp::Resolvable(Resolution::Resolved(Container(
                arg, resolved,
            ))));
        } else {
            first_pass.push(Timestamp::Unresolvable(arg));
        }
    }
    let mut second_pass = Vec::new();
    for timestamp in first_pass.into_iter() {
        match timestamp {
            Timestamp::Resolvable(Resolution::Later(arg)) => {
                let clean_arg = arg
                    .strip_suffix('!')
                    .unwrap_or(&arg);
                let is_minused;
                let duration_string = if let Some(plussed) = clean_arg.strip_prefix('+') {
                    is_minused = false;
                    plussed
                } else if let Some(minused) = clean_arg.strip_prefix('-') {
                    is_minused = true;
                    minused
                } else {
                    unreachable!("we checked if thing starts with +/- prior")
                };
                let mut targetted_timestamp = {
                    second_pass
                        .iter()
                        .rev()
                        .filter_map(|situation| {
                            if let Timestamp::Resolvable(Resolution::Resolved(Container(_, timestamp))) =
                                situation
                            {
                                Some(*timestamp)
                            } else {
                                None
                            }
                        })
                };
                let timedelta = parse_duration(duration_string);
                if is_minused {
                    let resulting_timestamp = targetted_timestamp
                        .next_back()
                        .unwrap_or(now)
                        - timedelta;
                    second_pass.insert(
                        0,
                        Timestamp::Resolvable(Resolution::Resolved(Container(arg, resulting_timestamp))),
                    );
                } else {
                    let resulting_timestamp = targetted_timestamp
                        .next()
                        .unwrap_or(now)
                        + timedelta;
                    second_pass.push(Timestamp::Resolvable(Resolution::Resolved(Container(
                        arg,
                        resulting_timestamp,
                    ))));
                }
            },
            other => second_pass.push(other),
        }
    }
    second_pass
}

fn parse_duration(duration: &str) -> TimeDelta {
    let mut hours = String::new();
    let mut minutes = String::new();
    let mut seconds = String::new();
    let mut intermediary = Vec::new();
    for ch in duration.chars() {
        match ch {
            'h' => {
                intermediary
                    .drain(..)
                    .for_each(|digit| hours.push(digit));
            },
            'm' => {
                intermediary
                    .drain(..)
                    .for_each(|digit| minutes.push(digit));
            },
            's' => {
                intermediary
                    .drain(..)
                    .for_each(|digit| seconds.push(digit));
            },
            other => {
                intermediary.push(other);
            },
        }
    }
    intermediary
        .drain(..)
        .for_each(|digit| seconds.push(digit));
    let hours: i64 = if hours.is_empty().not() {
        hours.parse().unwrap()
    } else {
        0
    };
    let minutes: i64 = if minutes.is_empty().not() {
        minutes.parse().unwrap()
    } else {
        0
    };
    let seconds: i64 = if seconds.is_empty().not() {
        seconds.parse().unwrap()
    } else {
        0
    };
    TimeDelta::hours(hours) + TimeDelta::minutes(minutes) + TimeDelta::seconds(seconds)
}

fn resolve_colon(colon_separated: &str, now: DateTime<Local>) -> DateTime<Local> {
    let colon_separated = colon_separated
        .strip_suffix('!')
        .unwrap_or(colon_separated);
    let mut the = colon_separated.split(':');
    let hour: u32 = the
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let minute: u32 = the
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let target_date = now
        .with_hour(hour)
        .unwrap()
        .with_minute(minute)
        .unwrap()
        .with_second(0)
        .unwrap();
    if target_date < now {
        target_date + Days::new(1)
    } else {
        target_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn christianity() {
        let input = vec!["1h20m".into(), "10m".into()];
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 23, 59, 59)
            .single()
            .unwrap();
        let output = resolve(input, now);
        assert_eq!(
            output[..],
            vec![
                Timestamp::Unresolvable("1h20m".into()),
                Timestamp::Unresolvable("10m".into())
            ]
        )
    }

    #[test]
    fn fairy() {
        let input = vec!["+1h20m".into(), "+10m".into()];
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 10, 00, 59)
            .single()
            .unwrap();
        let output = resolve(input, now);
        assert_eq!(
            output[..],
            vec![
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "+1h20m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 11, 20, 59)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "+10m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 11, 30, 59)
                        .single()
                        .unwrap()
                )))
            ]
        )
    }

    #[test]
    fn condos() {
        let input = vec!["12:00".into(), "+1h20m".into(), "+10m".into()];
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 10, 00, 59)
            .single()
            .unwrap();
        let output = resolve(input, now);
        assert_eq!(
            output[..],
            vec![
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "12:00".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 12, 00, 00)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "+1h20m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 13, 20, 00)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "+10m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 13, 30, 00)
                        .single()
                        .unwrap()
                )))
            ]
        )
    }

    #[test]
    fn attraction() {
        let input = vec!["12:00".into(), "-10m".into(), "-1h30m".into(), "-1h".into()];
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 10, 00, 59)
            .single()
            .unwrap();
        let output = resolve(input, now);
        assert_eq!(
            output[..],
            vec![
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "-1h".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 9, 20, 00)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "-1h30m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 10, 20, 00)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "-10m".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 11, 50, 00)
                        .single()
                        .unwrap()
                ))),
                Timestamp::Resolvable(Resolution::Resolved(Container(
                    "12:00".into(),
                    Local
                        .with_ymd_and_hms(2025, 12, 31, 12, 00, 00)
                        .single()
                        .unwrap()
                ))),
            ]
        )
    }

    #[test]
    fn visible() {
        let input = "23:00";
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 22, 59, 59)
            .single()
            .unwrap();
        let output = resolve_colon(input, now);
        assert_eq!(
            output,
            Local
                .with_ymd_and_hms(2025, 12, 31, 23, 00, 00)
                .single()
                .unwrap()
        )
    }

    #[test]
    fn cheese() {
        let input = "01:00";
        let now = Local
            .with_ymd_and_hms(2025, 12, 31, 23, 59, 59)
            .single()
            .unwrap();
        let output = resolve_colon(input, now);
        assert_eq!(
            output,
            Local
                .with_ymd_and_hms(2026, 1, 1, 01, 00, 00)
                .single()
                .unwrap()
        )
    }

    #[test]
    fn votes() {
        let input = "1h20m30s";
        let output = parse_duration(input);
        assert_eq!(
            output,
            TimeDelta::hours(1) + TimeDelta::minutes(20) + TimeDelta::seconds(30)
        )
    }

    #[test]
    fn institutions() {
        let input = "1h20m30";
        let output = parse_duration(input);
        assert_eq!(
            output,
            TimeDelta::hours(1) + TimeDelta::minutes(20) + TimeDelta::seconds(30)
        )
    }

    #[test]
    fn soccer() {
        let input = "1h30";
        let output = parse_duration(input);
        assert_eq!(output, TimeDelta::hours(1) + TimeDelta::seconds(30))
    }
}
