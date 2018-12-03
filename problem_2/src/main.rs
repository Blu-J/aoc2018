extern crate im;

use im::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
fn input() -> Result<String> {
    let file = File::open("inputs/part1")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn hash_part(line: &str) -> (i32, i32) {
    let mut hash_map: HashMap<char, i32> = HashMap::new();

    for c in line.chars() {
        hash_map
            .entry(c)
            .and_modify(|x| {
                *x += 1;
            }).or_insert(1);
    }
    (
        hash_map.values().filter(|&&value| value == 2).count() as i32,
        hash_map.values().filter(|&&value| value == 3).count() as i32,
    )
}

#[test]
fn test_hash_part() {
    assert_eq!((0, 0), hash_part(&"abcdef"));
    assert_eq!((1, 1), hash_part(&"bababc"));
    assert_eq!((1, 0), hash_part(&"abbcde"));
    assert_eq!((0, 1), hash_part(&"abcccd"));
    assert_eq!((2, 0), hash_part(&"aabcdd"));
    assert_eq!((1, 0), hash_part(&"abcdee"));
    assert_eq!((0, 2), hash_part(&"ababab"));
}

fn part_1(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let hashed = hash_part(&line.trim());
        twos += match hashed.0 {
            0 => 0,
            _ => 1,
        };
        threes += match hashed.1 {
            0 => 0,
            _ => 1,
        };
    }
    twos * threes
}
#[test]
fn test_part_1() {
    assert_eq!(
        12,
        part_1(&"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab")
    );
}

fn part_2(input: &str) -> String {
    let mut seen: HashMap<usize, HashMap<char, HashSet<String>>> = HashMap::new();
    let mut winner: String = String::new();

    for line in input.lines() {
        // Closeness
        let mut closeness_hash: HashMap<String, String> = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            let mut local_seen = seen.entry(i).or_insert_with(|| HashMap::new());
            let mut local_seen_set = local_seen.entry(c).or_insert_with(|| HashSet::new());
            for found_value in local_seen_set.iter() {
                let mut current_matches = closeness_hash
                    .entry(found_value.to_string())
                    .or_insert_with(|| String::new());
                current_matches.push(c);
            }
            local_seen_set.insert(line.to_string());
        }

        for (_key, value) in closeness_hash {
            if value.len() > winner.len() {
                winner = value.to_string();
            }
        }
    }

    winner
}

#[test]
fn test_part_2() {
    assert_eq!(
        "fgij",
        part_2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz")
    );
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read);
    let answer_part_2 = part_2(&input_read);

    println!("Answer for part 1 is {}", answer_part_1);
    println!("Answer for part 2 is {:?}", answer_part_2);
    Ok(())
}
