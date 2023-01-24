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

fn part1(input: &str) -> Result<()> {
    let messages: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let cols = messages[0].len();
    let mut counts: Vec<char> = vec!['\0'; cols];
    for c in 0..cols {
        let mut popular = HashMap::new();
        for m in &messages {
            let c = m.chars().nth(c).unwrap();
            *popular.entry(c).or_insert(0) += 1;
        }

        let p = popular.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
        counts[c] = *p.0;
    }

    let message: String = counts.iter().collect();
    println!("{message}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let messages: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let cols = messages[0].len();
    let mut counts: Vec<char> = vec!['\0'; cols];
    for c in 0..cols {
        let mut least_popular = HashMap::new();
        for m in &messages {
            let c = m.chars().nth(c).unwrap();
            *least_popular.entry(c).or_insert(0) += 1;
        }

        let p = least_popular.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
        counts[c] = *p.0;
    }

    let message: String = counts.iter().collect();
    println!("{message}");
    Ok(())
}
