extern crate im;

use im::HashMap;
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

fn hash_part(line: &String) -> (i32, i32) {
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
    assert_eq!((0, 0), hash_part(&"abcdef".into()));
    assert_eq!((1, 1), hash_part(&"bababc".into()));
    assert_eq!((1, 0), hash_part(&"abbcde".into()));
    assert_eq!((0, 1), hash_part(&"abcccd".into()));
    assert_eq!((2, 0), hash_part(&"aabcdd".into()));
    assert_eq!((1, 0), hash_part(&"abcdee".into()));
    assert_eq!((0, 2), hash_part(&"ababab".into()));
}

fn part_1(input: &String) -> Result<i32> {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let hashed = hash_part(&line.trim().into());
        twos += match hashed.0 {
            0 => 0,
            _ => 1,
        };
        threes += match hashed.1 {
            0 => 0,
            _ => 1,
        };
    }
    Ok(twos * threes)
}
#[test]
fn test_part_1() {
    assert_eq!(
        12,
        part_1(&String::from(
            "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"
        )).unwrap()
    );
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read)?;

    println!("Answer for part 1 is {}", answer_part_1);
    Ok(())
}
