use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

struct Triangle {
    side1: usize,
    side2: usize,
    side3: usize,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        if self.side1 + self.side2 <= self.side3 {
            false
        } else if self.side2 + self.side3 <= self.side1 {
            false
        } else if self.side3 + self.side1 <= self.side2 {
            false
        } else {
            true
        }
    }
}

impl From<&str> for Triangle {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.trim().split_whitespace().collect();

        Self {
            side1: parts[0].parse().unwrap(),
            side2: parts[1].parse().unwrap(),
            side3: parts[2].parse().unwrap(),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let triangles: Vec<Triangle> = input.lines().map(|l| l.into()).collect();

    let valid = triangles.iter().filter(|t| t.is_valid()).count();

    println!("Valid Triangles (part 1): {valid}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut col1: Vec<usize> = vec![];
    let mut col2: Vec<usize> = vec![];
    let mut col3: Vec<usize> = vec![];

    input.lines().for_each(|l| {
        let parts: Vec<&str> = l.trim().split_whitespace().collect();
        col1.push(parts[0].parse().unwrap());
        col2.push(parts[1].parse().unwrap());
        col3.push(parts[2].parse().unwrap());
    });

    let mut valid = 0;

    valid += valid_triangles(&col1);
    valid += valid_triangles(&col2);
    valid += valid_triangles(&col3);

    println!("Valid Triangles (part 2): {valid}");
    Ok(())
}

fn valid_triangles(vals: &Vec<usize>) -> usize {
    let mut valid = 0;
    for n in vals.as_slice().windows(3).step_by(3) {
        // println!("{:?}", n);
        let mut iter = n.into_iter().cloned();
        let t = Triangle {
            side1: iter.next().unwrap(),
            side2: iter.next().unwrap(),
            side3: iter.next().unwrap(),
        };

        if t.is_valid() {
            valid += 1;
        }
    }

    valid
}
