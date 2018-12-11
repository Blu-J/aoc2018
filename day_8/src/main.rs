use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

type Result<T> = ::std::result::Result<T, String>;

fn main() -> Result<()> {
    let input = read_input()?;
    let inputs: Vec<u32> = input
        .split_whitespace()
        .map(u32::from_str)
        .map(|x| x.map_err(|x| format!("Could not transform into number {}", x)))
        .collect::<Result<_>>()?;
    println!("Part 1 {}", part_1(&mut inputs.clone())?);
    println!("Part 2 {}", part_2(&mut inputs.clone())?);
    Ok(())
}

fn read_input() -> Result<String> {
    let file = File::open("input").map_err(|x| format!("Could not open file: {}", x))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .map_err(|x| format!("Could not read to string: {}", x))?;
    Ok(contents)
}

fn part_1(input: &mut Vec<u32>) -> Result<u32> {
    if input.len() < 2 {
        return Err(format!(
            "Don't have enough for getting the headers out: {:?}",
            input
        ));
    }
    let children_count = input.remove(0);
    let meta_count = input.remove(0) as usize;
    let child_answer = (0..children_count)
        .map(|_| part_1(input))
        .sum::<Result<u32>>()?;

    Ok(meta_count_total(input, meta_count)? + child_answer)
}

fn meta_count_total(input: &mut Vec<u32>, meta_count: usize) -> Result<u32> {
    if input.len() < meta_count as usize {
        return Err(format!(
            "Input={:?} is not long enough to drain {}",
            input, meta_count
        ));
    }
    Ok(input.drain(0..meta_count).sum())
}

#[test]
fn t_part_1() {
    let mut input: Vec<u32> = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(138, part_1(&mut input).unwrap());
}

fn part_2(input: &mut Vec<u32>) -> Result<u32> {
    if input.len() < 2 {
        return Err(format!(
            "Don't have enough for getting the headers out: {:?}",
            input
        ));
    }
    let children_count = input.remove(0);
    let meta_count = input.remove(0) as usize;
    let children_values: Vec<u32> = (0..children_count)
        .map(|_| part_2(input))
        .collect::<Result<_>>()?;
    if children_values.len() == 0 {
        return meta_count_total(input, meta_count);
    }
    if input.len() < meta_count {
        return Err(format!(
            "Input={:?} is not long enough to drain {}",
            input, meta_count
        ));
    }
    let drained = input.drain(0..meta_count);
    Ok(drained
        .filter_map(|i| children_values.get((i - 1) as usize))
        .sum())
}

#[test]
fn t_part_2() {
    let mut input: Vec<u32> = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(66, part_2(&mut input).unwrap());
}
