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
    let mut answer = 0;
    if input.len() < 2 {
        return Err(format!(
            "Don't have enough for getting the headers out: {:?}",
            input
        ));
    }
    let mut children_count = input.remove(0);
    let mut meta_count = input.remove(0);
    if input.len() < (children_count + meta_count) as usize {
        return Err(format!(
            "Don't have enough for getting the rest out: {:?} with child={} and meta={}",
            input, children_count, meta_count
        ));
    }
    loop {
        if children_count <= 0 {
            break;
        }
        children_count -= 1;
        answer += part_1(input)?;
    }

    loop {
        if meta_count <= 0 {
            break;
        }
        meta_count -= 1;
        answer += input.remove(0);
    }

    Ok(answer)
}

#[test]
fn t_part_1() {
    let mut input: Vec<u32> = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(138, part_1(&mut input).unwrap());
}

fn part_2(input: &mut Vec<u32>) -> Result<u32> {
    let mut answer = 0;
    if input.len() < 2 {
        return Err(format!(
            "Don't have enough for getting the headers out: {:?}",
            input
        ));
    }
    let mut children_count = input.remove(0);
    let mut meta_count = input.remove(0);
    if input.len() < (children_count + meta_count) as usize {
        return Err(format!(
            "Don't have enough for getting the rest out: {:?} with child={} and meta={}",
            input, children_count, meta_count
        ));
    }
    let mut children_values: Vec<u32> = Vec::new();
    loop {
        if children_count <= 0 {
            break;
        }
        children_count -= 1;
        children_values.push(part_2(input)?);
    }
    if children_values.len() == 0 {
        loop {
            if meta_count <= 0 {
                break;
            }
            meta_count -= 1;
            answer += input.remove(0);
        }
    } else {
        loop {
            if meta_count <= 0 {
                break;
            }
            meta_count -= 1;
            if let Some(child_value) = children_values.get((input.remove(0) - 1) as usize) {
                answer += child_value;
            }
        }
    }
    Ok(answer)
}

#[test]
fn t_part_2() {
    let mut input: Vec<u32> = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    assert_eq!(66, part_2(&mut input).unwrap());
}
