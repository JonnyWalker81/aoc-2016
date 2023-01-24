use std::{
    collections::HashMap,
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
struct Room {
    name: String,
    names: HashMap<char, usize>,
    number: usize,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut mapping: Vec<(&char, &usize)> = self.names.iter().collect();

        mapping.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        let sandbox: String = mapping.iter().take(5).map(|m| m.0).collect();

        sandbox == self.checksum
    }

    fn decrypt(&self) -> String {
        let mut result = String::new();
        for c in self.name.chars() {
            if c == '-' {
                result.push(' ');
            } else {
                let nn = (((c as u32).saturating_sub(97) + self.number as u32) % 26) + 97;
                let n = char::from_u32(nn).unwrap();
                result.push(n);
            }
        }

        result
    }
}

impl From<&str> for Room {
    fn from(item: &str) -> Self {
        let mut parts: Vec<&str> = item.split('-').collect();
        let (n, bracket) = parts.last().unwrap().split_once("[").unwrap();
        let number = n.parse().unwrap();
        let checksum = bracket[..bracket.len() - 1].to_string();
        let mut names = HashMap::new();
        parts.pop();
        let name = parts.join("-");
        for s in parts {
            for c in s.chars() {
                *names.entry(c).or_insert(0) += 1;
            }
        }

        Self {
            name,
            names,
            number,
            checksum,
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let rooms: Vec<Room> = input.lines().map(|l| l.into()).collect();

    // println!("{rooms:?}");
    // let count = rooms.iter().filter(|r| r.is_valid()).count();
    // println!("{}", count);

    let sum: usize = rooms
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| r.number)
        .sum();
    println!("Sum: {sum}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let rooms: Vec<Room> = input.lines().map(|l| l.into()).collect();

    // println!("{rooms:?}");
    // let count = rooms.iter().filter(|r| r.is_valid()).count();
    // println!("{}", count);

    let names: Vec<(String, usize)> = rooms
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| (r.decrypt(), r.number))
        .collect();

    let north = names.iter().find(|n| n.0.contains("north")).unwrap();
    println!("{}", north.1);
    Ok(())
}
