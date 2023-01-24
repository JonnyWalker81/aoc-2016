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
    L,
    R,
    U,
    D,
}

impl From<char> for Command {
    fn from(item: char) -> Self {
        match item {
            'L' => Self::L,
            'R' => Self::R,
            'U' => Self::U,
            'D' => Self::D,
            _ => panic!("should not get here: {}", item),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let commands: Vec<Vec<Command>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();

    let keypad = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let mut code = String::new();
    let mut pos = (1usize, 1usize);
    for (r, v) in commands.iter().enumerate() {
        let current = 0;
        for (col, c) in v.iter().enumerate() {
            let next = match c {
                Command::L => (pos.0 as i64, pos.1 as i64 - 1),
                Command::R => (pos.0 as i64, pos.1 as i64 + 1),
                Command::U => (pos.0 as i64 - 1, pos.1 as i64),
                Command::D => (pos.0 as i64 + 1, pos.1 as i64),
            };

            if next.0 < 0
                || next.0 >= keypad.len() as i64
                || next.1 < 0
                || next.1 >= keypad[0].len() as i64
            {
                continue;
            }

            pos.0 = next.0 as usize;
            pos.1 = next.1 as usize;
        }
        code.push_str(&keypad[pos.0][pos.1].to_string());
    }

    println!("{code:?}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let commands: Vec<Vec<Command>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect();

    let keypad = vec![
        vec!["0", "0", "1", "0", "0"],
        vec!["0", "2", "3", "4", "0"],
        vec!["5", "6", "7", "8", "9"],
        vec!["0", "A", "B", "C", "0"],
        vec!["0", "0", "D", "0", "0"],
    ];

    let mut code = String::new();
    let mut pos = (2usize, 0usize);
    for v in commands.iter() {
        for c in v.iter() {
            let next = match c {
                Command::L => (pos.0 as i64, pos.1 as i64 - 1),
                Command::R => (pos.0 as i64, pos.1 as i64 + 1),
                Command::U => (pos.0 as i64 - 1, pos.1 as i64),
                Command::D => (pos.0 as i64 + 1, pos.1 as i64),
            };

            println!("before: {pos:?}, after: {:?}", next);
            if next.0 < 0
                || next.0 >= keypad.len() as i64
                || next.1 < 0
                || next.1 >= keypad[next.0 as usize].len() as i64
                || keypad[next.0 as usize][next.1 as usize] == "0"
            {
                continue;
            }

            pos.0 = next.0 as usize;
            pos.1 = next.1 as usize;
        }
        println!("{pos:?}");
        println!("{}", keypad[pos.0][pos.1]);
        code.push_str(keypad[pos.0][pos.1]);
    }

    println!("{code:?}");

    Ok(())
}
