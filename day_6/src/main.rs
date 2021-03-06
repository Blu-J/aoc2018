use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{cmp, error::Error, str::FromStr};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
type R<A, B> = ::std::result::Result<A, B>;
type Id = usize;

fn main() -> Result<()> {
    let input = read_input()?;
    // let coords: Vec<_> = input.lines().map(|x| x.parse()).collect::<R<_, _>>()?;
    println!("Answer is {}", part_1(&input)?);
    println!("Answer 2 is {}", part_2(&input, 10000)?);
    Ok(())
}

fn read_input() -> Result<String> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
fn manhattan_distance(a: &Coord, b: &Coord) -> u32 {
    (((a.x as i32) - (b.x as i32)).abs() + ((a.y as i32) - (b.y as i32)).abs()) as u32
}

#[test]
fn test_manhattan() {
    let p1 = ("1, 1").parse().unwrap();
    let p2 = ("1, 2").parse().unwrap();
    let p3 = ("2, 2").parse().unwrap();
    let p4 = ("4, 4").parse().unwrap();
    assert_eq!(0, manhattan_distance(&p1, &p1));
    assert_eq!(1, manhattan_distance(&p1, &p2));
    assert_eq!(2, manhattan_distance(&p1, &p3));
    assert_eq!(6, manhattan_distance(&p1, &p4));
}

#[derive(Eq, PartialEq, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn empty() -> Coord {
        Coord { x: 0, y: 0 }
    }

    fn empty_max() -> Coord {
        Coord {
            x: i64::max_value(),
            y: i64::max_value(),
        }
    }

    fn expand_bound(mut self, other: &Coord) -> Coord {
        self.x = cmp::max(self.x, other.x);
        self.y = cmp::max(self.y, other.y);
        self
    }
    fn decrease_bound(mut self, other: &Coord) -> Coord {
        self.x = cmp::min(self.x, other.x);
        self.y = cmp::min(self.y, other.y);
        self
    }
}

impl FromStr for Coord {
    type Err = Box<Error>;

    fn from_str(value: &str) -> Result<Coord> {
        let values = value
            .split(", ")
            .map(|x| x.parse::<usize>())
            .collect::<::std::result::Result<Vec<usize>, _>>()?;
        if values.len() != 2 {
            return Err("Could not parse the input".into());
        }
        Ok(Coord {
            x: values[0] as i64,
            y: values[1] as i64,
        })
    }
}

fn part_1(input: &str) -> Result<u32> {
    let coords: Vec<_> = input.lines().map(|x| x.parse()).collect::<R<_, _>>()?;
    coords_most_area(&coords)
}

fn coords_most_area(coords: &[Coord]) -> Result<u32> {
    let bounds = coords.iter().fold(Coord::empty(), |x, y| x.expand_bound(y));

    let lower_bounds = coords
        .iter()
        .fold(Coord::empty_max(), |x, y| x.decrease_bound(y));

    let with_ids: Vec<(Id, &Coord)> = coords.iter().enumerate().collect();

    let answer: Vec<Vec<_>> = (0..=bounds.x)
        .map(|x| {
            (0..=bounds.y)
                .map(|y| {
                    let current = Coord { x, y };
                    let with_distance: Vec<_> = with_ids
                        .iter()
                        .map(|(id, coord)| (id, manhattan_distance(&current, coord)))
                        .collect();
                    if let Some((&lowest_id, lowest_dist)) =
                        with_distance.iter().min_by_key(|(_, dist)| *dist)
                    {
                        match with_distance
                            .iter()
                            .filter(|(_, dist)| *dist == *lowest_dist)
                            .count()
                        {
                            1 => Some(lowest_id),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    // Collect totals
    type Count = u32;
    let mut totals: HashMap<Id, Count> = HashMap::new();
    for column in answer.iter() {
        for cell in column {
            if let Some(id) = cell {
                totals.entry(*id).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    // Remove the infinite (the endges)
    for (x, column) in answer.iter().enumerate() {
        for (y, cell) in column.iter().enumerate() {
            if let Some(id) = cell {
                if (x as i64) <= lower_bounds.x
                    || (y as i64) <= lower_bounds.y
                    || (x as i64) >= bounds.x
                    || (y as i64) >= bounds.y
                {
                    totals.remove(id);
                }
            }
        }
    }

    let total = match totals.iter().max_by_key(|(_, area)| *area) {
        Some((_, total)) => *total,
        _ => {
            println!("We have no totals");
            0
        }
    };

    Ok(total)
}

fn part_2(input: &str, max_distance: u32) -> Result<u32> {
    let coords: Vec<_> = input.lines().map(|x| x.parse()).collect::<R<_, _>>()?;
    coords_sum(&coords, max_distance)
}

fn coords_sum(coords: &[Coord], max_distance: u32) -> Result<u32> {
    let bounds = coords.iter().fold(Coord::empty(), |x, y| x.expand_bound(y));
    let offset = 32;
    let extra_size = offset * 2;

    let answer: Vec<Vec<u32>> = (0..=(bounds.x + extra_size))
        .map(|x| {
            (0..=(bounds.y + extra_size))
                .map(|y| {
                    let current = Coord {
                        x: x - offset,
                        y: y - offset,
                    };
                    let with_distance = coords
                        .iter()
                        .map(|coord| manhattan_distance(&current, coord))
                        .sum();
                    with_distance
                })
                .collect()
        })
        .collect();

    let mut total = 0;
    for column in answer {
        for cell in column {
            if cell < max_distance {
                total += 1;
            }
        }
    }
    Ok(total)
}

#[test]
fn test_part_1() {
    let input = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;
    let input_2 = r#"0, 0
10, 0
9, 9"#;
    assert_eq!(17, part_1(&input).unwrap());
    assert_eq!(0, part_1(&input_2).unwrap());
}

#[test]
fn test_part_2() {
    let input = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;
    let input_2 = r#"0, 0
10, 0
9, 9"#;
    assert_eq!(16, part_2(&input, 32).unwrap());
    // assert_eq!(0, part_2(&input_2, 32).unwrap());
}
