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

struct Part1 {
    value: Vec<char>,
}

fn opposite(value: char) -> char {
    if value.is_uppercase() {
        value.to_lowercase().nth(0).unwrap()
    } else {
        value.to_uppercase().nth(0).unwrap()
    }
}

impl Part1 {
    fn add_char(self, new_value: char) -> Part1 {
        match self.value.as_slice().split_first() {
            None => Part1 {
                value: vec![new_value],
            },
            Some((&head, tail)) => {
                if opposite(new_value) == head {
                    Part1 { value: tail.into() }
                } else {
                    let mut value = vec![new_value, head];
                    let mut tail_vec: Vec<char> = tail.into();
                    value.append(&mut tail_vec);
                    Part1 { value }
                }
            }
        }
    }
}

fn part_1(input: &str) -> Result<u32> {
    let mut answer = Part1 { value: vec![] };
    for c in input.chars() {
        answer = answer.add_char(c);
    }
    Ok(answer.value.len() as u32)
}

#[test]
fn test_part_1() {
    assert_eq!(0, part_1("aA").unwrap());
    assert_eq!(0, part_1("abBA").unwrap());
    assert_eq!(4, part_1("abAB").unwrap());
    assert_eq!(6, part_1("aabAAB").unwrap());
}

fn part_2(input: &str) -> Result<u32> {
    let mut answer = Part1 { value: vec![] };
    for c in input.chars() {
        answer = answer.add_char(c);
    }
    for remove_char in "abcdefghijklmnopqrstuvwxyz".chars() {
        let opposite_remove_char = opposite(remove_char);
        let mut local_answer = Part1 { value: vec![] };
        for c in input.chars() {
            if !(c == remove_char || c == opposite_remove_char) {
                local_answer = local_answer.add_char(c);
            }
        }
        if answer.value.len() > local_answer.value.len() {
            answer = local_answer;
        }
    }
    Ok(answer.value.len() as u32)
}

fn main() -> Result<()> {
    let input_read = input()?;
    let answer_part_1 = part_1(&input_read)?;

    println!("Answer for part 1 is {}", answer_part_1);
    let answer_part_2 = part_2(&input_read)?;

    println!("Answer for part 2 is {}", answer_part_2);
    Ok(())
}
