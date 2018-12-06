// extern crate im;
#[macro_use]
extern crate lazy_static;
extern crate regex;

// use im::{HashMap, HashSet};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
fn input() -> Result<String> {
    let file = File::open("inputs/part1")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn part_1(input: &str) -> Result<u32> {
    unimplemented!();
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read)?;

    println!("Answer for part 1 is {}", answer_part_1);
    Ok(())
}

type Id = u32;

#[derive(Debug, Eq, PartialEq)]
struct Event {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    kind: Kind,
}

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    StartShift { id: Id },
    Sleep,
    Wake,
}

impl FromStr for Event {
    type Err = Box<Error>;

    fn from_str(value: &str) -> Result<Event> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                    (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                    \s+
                    (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]
                \s+
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
            "
            ).unwrap();
        }

        let caps = match RE.captures(value) {
            None => return Err("unrecognized event".into()),
            Some(caps) => caps,
        };
        let kind = if let Some(m) = caps.name("id") {
            Kind::StartShift {
                id: m.as_str().parse()?,
            }
        } else if &caps["sleep"] == "falls asleep" {
            Kind::Sleep
        } else if &caps["sleep"] == "wakes up" {
            Kind::Wake
        } else {
            return Err("could not determine event kind".into());
        };
        Ok(Event {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
            kind,
        })
    }
}

const input_1: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;
#[test]
fn test_1() {
    assert_eq!(240, part_1(input_1).unwrap());
}
#[test]
fn test_parse_line() {
    assert_eq!(
        Event {
            year: 1518,
            month: 11,
            day: 1,
            hour: 0,
            minute: 0,
            kind: Kind::StartShift { id: 10 },
        },
        "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap()
    );
    assert_eq!(
        Event {
            year: 1518,
            month: 11,
            day: 1,
            hour: 0,
            minute: 5,
            kind: Kind::Sleep,
        },
        "[1518-11-01 00:05] falls asleep".parse().unwrap()
    );
    assert_eq!(
        Event {
            year: 1518,
            month: 11,
            day: 1,
            hour: 0,
            minute: 25,
            kind: Kind::Wake,
        },
        "[1518-11-01 00:25] wakes up".parse().unwrap()
    );
}
