#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Part 1 {}", part_1(&input)?);
    println!("Part 2 {}", part_2(&input, 5, 60)?);
    Ok(())
}

fn read_input() -> Result<String> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Relationship {
    node: String,
    require: String,
}
impl FromStr for Relationship {
    type Err = Box<::std::error::Error>;

    fn from_str(line: &str) -> Result<Relationship> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Step (?P<require>\w+) must be finished before step (?P<node>\w+) can begin."
            )
            .unwrap();
        }

        let caps = match RE.captures(line.trim()) {
            None => return Err("Unrecognized relationship".into()),
            Some(caps) => caps,
        };

        Ok(Relationship {
            node: caps["node"].into(),
            require: caps["require"].into(),
        })
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        Relationship {
            node: "A".into(),
            require: "C".into(),
        },
        "Step C must be finished before step A can begin."
            .parse()
            .unwrap()
    );
}

fn part_1(input: &str) -> Result<String> {
    let relationships: Vec<Relationship> = input
        .lines()
        .map(Relationship::from_str)
        .map(|x| x.unwrap())
        .collect();

    // Construct are relationship mapping of Node -> Set Nodes
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for relationship in relationships {
        map.entry(relationship.require.clone())
            .or_insert_with(HashSet::new);
        let entry = map.entry(relationship.node).or_insert_with(HashSet::new);
        entry.insert(relationship.require);
    }

    let mut answer = String::new();
    loop {
        // Our ending condition is when we no longer have a map to pull
        // out of
        if map.len() == 0 {
            return Ok(answer);
        }

        // Find the head that is sorted
        let first_node_head: Option<String> = map
            .iter()
            .filter(|(_, requirements)| requirements.len() == 0)
            .map(|(node, _)| node)
            .cloned()
            .min();

        if let Some(head) = first_node_head {
            // Add head to answer
            answer += &head;
            // Remove from mapping
            map.remove(&head);
            // Remove the head from all the other required nodes
            map.iter_mut().for_each(|(_, requirements)| {
                requirements.remove(&head);
            });
        }
    }
}

fn part_2(input: &str, max_workers: usize, completion_time: u32) -> Result<u32> {
    let relationships: Vec<Relationship> = input
        .lines()
        .map(Relationship::from_str)
        .map(|x| x.unwrap())
        .collect();

    // Construct relation map Node -> Set Node
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for relationship in relationships {
        map.entry(relationship.require.clone())
            .or_insert_with(HashSet::new);
        let entry = map.entry(relationship.node).or_insert_with(HashSet::new);
        entry.insert(relationship.require);
    }

    let mut turns = 0;
    let mut workers: HashMap<String, u32> = HashMap::new();
    loop {
        // Add in all the head to open workers
        loop {
            if workers.len() >= max_workers {
                break;
            }
            let first_node_head: Option<String> = map
                .iter()
                .filter(|(_, requirements)| requirements.len() == 0)
                .map(|(node, _)| node)
                .cloned()
                .min();
            match first_node_head {
                Some(head) => {
                    map.remove(&head);
                    workers.insert(head, turns);
                }
                _ => {
                    break;
                }
            }
        }
        // Finnish Condition is when the workers are doing nothing.
        if workers.len() == 0 {
            return Ok(turns);
        }
        // Split our workers that are still working and those that are done
        let (done_work, keep_working): (HashMap<_, _>, HashMap<_, _>) =
            workers.into_iter().partition(|(head, turn)| {
                (head.as_bytes()[0] as u32) - 65 + completion_time + *turn <= turns
            });
        // For the workers that are done remove the requirements from all the other nodes
        for (head, _) in done_work {
            map.iter_mut().for_each(|(_, requirements)| {
                requirements.remove(&head);
            });
        }
        workers = keep_working;
        turns += 1;
    }
}

#[test]
fn test_1() {
    let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;
    assert_eq!(&part_1(input).unwrap(), &"CABDFE");
}

#[test]
fn test_2() {
    let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;
    assert_eq!(part_2(input, 2, 0).unwrap(), 15);
    assert_eq!(part_2(input, 3, 0).unwrap(), 14);
    assert_eq!(part_2(input, 1, 0).unwrap(), 21);
}
