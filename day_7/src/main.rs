#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{cmp, error::Error, str::FromStr};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
type R<A, B> = ::std::result::Result<A, B>;
type Id = usize;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Part 1 {}", part_1(&input)?);
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

fn parse_relationship(line: &str) -> Result<Relationship> {
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

#[test]
fn test_parse() {
    assert_eq!(
        Relationship {
            node: "A".into(),
            require: "C".into(),
        },
        parse_relationship("Step C must be finished before step A can begin.").unwrap()
    );
}

fn sorted_hash(hash: HashSet<String>) -> Vec<String> {
    let mut answer: Vec<_> = hash.iter().cloned().collect();
    answer.sort();
    answer
}

fn part_1(input: &str) -> Result<String> {
    let relationships: Vec<Relationship> = input
        .lines()
        .map(parse_relationship)
        .map(|x| x.unwrap())
        .collect();

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for relationship in relationships {
        map.entry(relationship.require.clone())
            .or_insert_with(HashSet::new);
        let entry = map.entry(relationship.node).or_insert_with(HashSet::new);
        entry.insert(relationship.require);
    }

    let mut answer = String::new();

    loop {
        if map.len() == 0 {
            return Ok(answer);
        }
        let first_node_head: Option<String> = map
            .iter()
            .filter(|(_, requirements)| requirements.len() == 0)
            .map(|(node, _)| node)
            .cloned()
            .min();

        if let Some(head) = first_node_head {
            answer += &head;
            map.remove(&head);
            map.iter_mut().for_each(|(_, requirements)| {
                requirements.remove(&head);
            });
        }
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
