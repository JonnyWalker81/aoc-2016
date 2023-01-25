use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateCol(usize, usize),
    RotateRow(usize, usize),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        match parts[0] {
            "rect" => {
                let (col, row) = parts[1].split_once('x').unwrap();
                Self::Rect(row.parse().unwrap(), col.parse().unwrap())
            }
            "rotate" if parts[1] == "column" => {
                let (_, col) = parts[2].split_once('=').unwrap();
                Self::RotateCol(col.parse().unwrap(), parts.last().unwrap().parse().unwrap())
            }
            "rotate" if parts[1] == "row" => {
                let (_, row) = parts[2].split_once('=').unwrap();
                Self::RotateRow(row.parse().unwrap(), parts.last().unwrap().parse().unwrap())
            }
            _ => panic!("should not get here: {parts:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}

fn part1(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect();
    // println!("{instructions:?}");

    let mut screen = vec![vec![Pixel::Off; 50]; 6];
    for ins in instructions {
        match ins {
            Instruction::Rect(row, col) => {
                for r in 0..row {
                    for c in 0..col {
                        screen[r][c] = Pixel::On
                    }
                }
            }
            Instruction::RotateCol(col, shift) => {
                let mut shifted_pos = vec![];
                for r in 0..screen.len() {
                    if screen[r][col] == Pixel::On {
                        let next_row = (r + shift) % screen.len();
                        shifted_pos.push(next_row);
                    }
                }

                for r in 0..screen.len() {
                    screen[r][col] = Pixel::Off;
                }
                for r in shifted_pos {
                    screen[r][col] = Pixel::On;
                }
            }
            Instruction::RotateRow(row, shift) => {
                let mut shifted_pos = vec![];
                for c in 0..screen[row].len() {
                    if screen[row][c] == Pixel::On {
                        let next_col = (c + shift) % screen[row].len();
                        shifted_pos.push(next_col);
                    }
                }

                for c in 0..screen[row].len() {
                    screen[row][c] = Pixel::Off;
                }

                for c in shifted_pos {
                    screen[row][c] = Pixel::On;
                }
            }
        }
    }

    display(&screen);
    let on_count = screen.iter().flatten().filter(|p| **p == Pixel::On).count();

    println!("{on_count}");

    Ok(())
}

fn display(screen: &Vec<Vec<Pixel>>) {
    for r in 0..screen.len() {
        for c in 0..screen[r].len() {
            match screen[r][c] {
                Pixel::On => print!("#"),
                Pixel::Off => print!("."),
            }
        }
        println!();
    }
    println!();
}
