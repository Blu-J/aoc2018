extern crate hashbrown;
extern crate im;

use im::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
fn input() -> Result<String> {
    let file = File::open("input1")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn part_1(input: &String) -> Result<i32> {
    let mut amount = 0;
    for line in input.lines() {
        let value: i32 = line.parse()?;
        amount += value;
    }
    Ok(amount)
}

fn part_2(input: &String) -> Result<i32> {
    let mut amount = 0;
    let mut hash_set: HashSet<i32> = HashSet::new();
    loop {
        for line in input.lines() {
            let value: i32 = line.parse()?;
            amount += value;
            if hash_set.contains(&amount) {
                return Ok(amount);
            }
            hash_set.insert(amount);
        }
    }
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read)?;
    let answer_part_2 = part_2(&input_read)?;

    println!("Answer for part 1 is {}", answer_part_1);
    println!("Answer for part 2 is {}", answer_part_2);
    Ok(())
}
