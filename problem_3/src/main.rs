extern crate im;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use im::{HashMap, HashSet};
use regex::Regex;
use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
fn input() -> Result<String> {
    let file = File::open("inputs/part_1")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
struct Claim {
    offset_x: u32,
    offset_y: u32,
    size_x: u32,
    size_y: u32,
}
fn part_1(input: &str) -> Result<u32> {
    lazy_static! {
        static ref regex: Regex = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    let claims = input
        .lines()
        .map(|line| {
            let capture = regex.captures(line).unwrap();
            let offset_x = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let offset_y = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let size_x = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let size_y = capture.get(4).unwrap().as_str().parse::<u32>().unwrap();
            Claim {
                offset_x,
                offset_y,
                size_x,
                size_y,
            }
        }).collect::<Vec<Claim>>();

    let (max_x, max_y) = claims.iter().fold((0u32, 0u32), |acc, claim| {
        (
            max(acc.0, claim.offset_x + claim.size_x),
            max(acc.1, claim.offset_y + claim.size_y),
        )
    });

    let mut grid = vec![vec![0u32; (max_y + 2) as usize]; (max_x + 2) as usize];

    for claim in claims {
        for x in claim.offset_x..(claim.offset_x + claim.size_x) {
            for y in claim.offset_y..(claim.offset_y + claim.size_y) {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    let mut count = 0;

    for x_vec in grid {
        for y_count in x_vec {
            if y_count >= 2 {
                count += 1;
            }
        }
    }
    Ok(count)
}

const TEST_INPUT_1: &str = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;
#[test]
fn test_part_1() {
    assert_eq!(4, part_1(TEST_INPUT_1).unwrap());
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read)?;

    println!("Answer for part 1 is {}", answer_part_1);
    Ok(())
}
