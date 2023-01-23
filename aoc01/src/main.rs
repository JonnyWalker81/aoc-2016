use std::{
    collections::HashSet,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Command {
    Left(i64),
    Right(i64),
}

impl Command {
    fn value(&self) -> i64 {
        match &self {
            Self::Left(n) => *n,
            Self::Right(n) => *n,
        }
    }
}

impl From<&str> for Command {
    fn from(item: &str) -> Self {
        match &item[0..1] {
            "L" => Self::Left(item[1..].parse().unwrap()),
            "R" => Self::Right(item[1..].parse().unwrap()),
            _ => panic!("should not get here: {}", item),
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, command: &Command) -> Self {
        match self {
            Self::North => match command {
                Command::Left(_) => Self::West,
                Command::Right(_) => Self::East,
            },
            Self::South => match command {
                Command::Left(_) => Self::East,
                Command::Right(_) => Self::West,
            },
            Self::East => match command {
                Command::Left(_) => Self::North,
                Command::Right(_) => Self::South,
            },
            Self::West => match command {
                Command::Left(_) => Self::South,
                Command::Right(_) => Self::North,
            },
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let directions: Vec<Command> = input.trim().split(", ").map(|d| d.into()).collect();

    // println!("{directions:?}");

    let mut pos = (0i64, 0i64);
    let mut current_direction = Direction::North;
    for d in directions {
        let next = current_direction.turn(&d);
        let n = d.value();
        match next {
            Direction::North => pos.0 -= n,
            Direction::South => pos.0 += n,
            Direction::East => pos.1 -= n,
            Direction::West => pos.1 += n,
        }
        current_direction = next;
    }

    println!("Pos: {pos:?}");

    let distance = manhattan((0, 0), pos);

    println!("Distance: {distance}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let directions: Vec<Command> = input.trim().split(", ").map(|d| d.into()).collect();

    // println!("{directions:?}");

    let mut pos = (0i64, 0i64);
    let mut current_direction = Direction::North;
    let mut visited = HashSet::new();
    visited.insert(pos);
    'outter: for d in directions {
        let next = current_direction.turn(&d);
        let n = d.value();
        let current = pos;
        match next {
            Direction::North => {
                let end = pos.0 - n;
                for i in end + 1..pos.0 {
                    if visited.contains(&(i, current.1)) {
                        println!("found twice: {:?}", (i, current.1));
                        let distance = manhattan((0, 0), (i, current.1));
                        println!("Distance: {}", distance);
                        break 'outter;
                    }
                    visited.insert((i, current.1));
                }
                pos.0 -= n;
            }
            Direction::South => {
                let end = pos.0 + n;
                for i in pos.0 + 1..end {
                    if visited.contains(&(i, current.1)) {
                        println!("found twice: {:?}", (i, current.1));
                        let distance = manhattan((0, 0), (i, current.1));
                        println!("Distance: {}", distance);
                        break 'outter;
                    }
                    visited.insert((i, current.1));
                }
                pos.0 += n;
            }
            Direction::East => {
                let end = pos.1 + n;
                for i in pos.1 + 1..end {
                    if visited.contains(&(current.0, i)) {
                        println!("found twice: {:?}", (current.0, i));
                        let distance = manhattan((0, 0), (current.0, i));
                        println!("Distance: {}", distance);
                        break 'outter;
                    }
                    visited.insert((current.0, i));
                }
                pos.1 += n;
            }
            Direction::West => {
                let end = pos.1 - n;
                for i in end + 1..pos.1 {
                    if visited.contains(&(current.0, i)) {
                        println!("found twice: {:?}", (current.0, i));
                        let distance = manhattan((0, 0), (current.0, i));
                        println!("Distance: {}", distance);
                        break 'outter;
                    }
                    visited.insert((current.0, i));
                }
                pos.1 -= n;
            }
        }

        visited.insert(pos);

        current_direction = next;
    }

    Ok(())
}

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
}
