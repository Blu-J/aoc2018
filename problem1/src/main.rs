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

fn part_1(input: &String) -> Result<()> {
    let mut amount = 0;
    for line in input.lines() {
        let value: i32 = line.parse()?;
        amount += value;
    }
    println!("Total amount is {}", amount);
    Ok(())
}

fn main() -> Result<()> {
    let input_read = input()?;
    part_1(&input_read)?;
    Ok(())
}
