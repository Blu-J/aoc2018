#[macro_use]
extern crate lazy_static;

use im::HashMap;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
fn input() -> Result<String> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
type SleepingAmount = HashMap<u32, u32>;

fn sort_events(input: &str) -> Vec<Event> {
    let mut events = input
        .lines()
        .map(|x| x.parse::<Event>())
        .filter_map(Result::ok)
        .collect::<Vec<Event>>();
    events.sort();
    events
}

type IdSleep = HashMap<Id, SleepingAmount>;
fn hash_event_times(events: &[Event]) -> IdSleep {
    let mut id_sleep: IdSleep = HashMap::new();
    let mut current_id: Option<Id> = None;
    let mut start_time: Option<Time> = None;
    for event in events {
        match event.kind {
            Kind::StartShift { id } => {
                current_id = Some(id);
                start_time = None;
            }
            Kind::Sleep => {
                let cloned_time = Time {
                    year: event.time.year,
                    month: event.time.month,
                    day: event.time.day,
                    hour: event.time.hour,
                    minute: event.time.minute,
                };
                start_time = Some(cloned_time);
            }
            Kind::Wake => {
                let value = id_sleep
                    .entry(current_id.unwrap())
                    .or_insert_with(HashMap::new);
                if let Some(start_time) = start_time {
                    for i in (start_time.minute)..event.time.minute {
                        let time_value = value.entry(i).or_insert(0);
                        *time_value += 1;
                    }
                }
                start_time = None;
            }
        }
    }
    id_sleep
}

fn part_1(id_sleep: &IdSleep) -> Result<u32> {
    // Find the biggest time
    let mut id_2_sleep: Vec<(Id, SleepingAmount, u32)> = id_sleep
        .iter()
        .cloned()
        .map(|(key, sleeping_time)| {
            let total_time = sleeping_time.values().cloned().sum::<u32>();
            (key, sleeping_time, total_time)
        })
        .collect();
    id_2_sleep.sort_by_key(|&(_, _, total_time)| total_time);
    id_2_sleep
        .last()
        .ok_or_else(|| "Could not find the first item".into())
        .map(|(left, sleeping_amount, _)| {
            let mut sleeping_amount: Vec<(u32, u32)> = sleeping_amount.iter().cloned().collect();
            sleeping_amount.sort_by_key(|(_, amount)| *amount);
            let found_time = sleeping_amount.last().unwrap();
            (*left) * (found_time.0)
        })
}
fn part_2(id_sleep: &IdSleep) -> Result<u32> {
    let with_highest_sleep = id_sleep.iter().cloned().map(|(key, sleeping_time)| {
        let total_time = sleeping_time
            .iter()
            .cloned()
            .max_by_key(|(_, sleep_count)| *sleep_count)
            .unwrap();
        (key, sleeping_time, total_time)
    });

    let last_item: (Id, (u32, u32)) =
        with_highest_sleep.fold((0, (0, 0)), |acc, (id, _, (max_hour, max_hour_count))| {
            let (_id, (_max_hour, acc_max_hour_count)) = acc;
            if max_hour_count > acc_max_hour_count {
                return (id, (max_hour, max_hour_count));
            }
            acc
        });
    let (id, (max_hour, _)) = last_item;
    Ok(id * max_hour)
}

fn main() -> Result<()> {
    let input_read = input()?;
    let events = sort_events(&input_read);
    let id_sleep = hash_event_times(&events);
    let answer_part_2 = part_2(&id_sleep)?;
    let answer_part_1 = part_1(&id_sleep)?;

    println!("Answer for part 1 is {:?}", answer_part_1);
    println!("Answer for part 2 is {:?}", answer_part_2);
    Ok(())
}

type Id = u32;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Event {
    time: Time,
    kind: Kind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Kind {
    StartShift { id: Id },
    Wake,
    Sleep,
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
            )
            .unwrap();
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
        let time = Time {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };
        Ok(Event { time, kind })
    }
}

#[test]
fn test_case() {
    const INPUT_1: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
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
    let events = sort_events(&INPUT_1);
    let id_sleep = hash_event_times(&events);
    assert_eq!(240, part_1(&id_sleep).unwrap());
    assert_eq!(4455, part_2(&id_sleep).unwrap());
}
#[test]
fn test_parse_line() {
    assert_eq!(
        Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0,
            },
            kind: Kind::StartShift { id: 10 },
        },
        "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap()
    );
    assert_eq!(
        Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 5,
            },
            kind: Kind::Sleep,
        },
        "[1518-11-01 00:05] falls asleep".parse().unwrap()
    );
    assert_eq!(
        Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 25,
            },
            kind: Kind::Wake,
        },
        "[1518-11-01 00:25] wakes up".parse().unwrap()
    );
}
