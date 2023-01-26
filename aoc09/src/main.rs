use std::{
    io::{self, Read},
    os::unix::process,
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
    let count = process(input);
    println!("count: {}", count);

    Ok(())
}

fn process(input: &str) -> usize {
    let input_chars: Vec<char> = input.chars().collect();
    let mut pos = 0;
    let mut count = 0;
    loop {
        match input_chars[pos] {
            '(' => {
                let (num_chars, repeat_count, p) = parse_compress_command(&input, pos + 1);
                let to_repeat = &input[p + 1..(p + 1) + num_chars];
                for _ in 0..repeat_count {
                    count += to_repeat.len();
                }

                pos = num_chars + p;
            }
            c @ _ => {
                if !c.is_whitespace() {
                    count += 1;
                }
            }
        }

        pos += 1;
        if pos >= input_chars.len() {
            break;
        }
    }

    count
}

fn process_rec(input: &str) -> usize {
    let input_chars: Vec<char> = input.chars().collect();
    let mut pos = 0;
    let mut count = 0;
    loop {
        match input_chars[pos] {
            '(' => {
                let (num_chars, repeat_count, p) = parse_compress_command(&input, pos + 1);
                let to_repeat: &Vec<char> = &input[p + 1..(p + 1) + num_chars].chars().collect();
                let mut c = to_repeat.len();
                if to_repeat[0] == '(' {
                    c = process_rec(&input[p + 1..(p + 1) + num_chars]);
                }

                for _ in 0..repeat_count {
                    count += c;
                }

                pos = num_chars + p;
            }
            c @ _ => {
                if !c.is_whitespace() {
                    count += 1;
                }
            }
        }

        pos += 1;
        if pos >= input_chars.len() {
            break;
        }
    }

    count
}

fn part2(input: &str) -> Result<()> {
    let count = process_rec(input);
    println!("count: {}", count);

    Ok(())
}

fn parse_number(input: &str, mut pos: usize) -> (usize, usize) {
    let mut num_chars = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            pos += 1;
            num_chars.push(c);
        } else {
            break;
        }
    }

    (num_chars.parse().unwrap_or(0), pos)
}

fn parse_compress_command(input: &str, pos: usize) -> (usize, usize, usize) {
    let (num_chars, p) = parse_number(&input[pos..], pos);
    if let Some(c) = input.chars().nth(p) {
        if c != 'x' {
            panic!("should be an x: {}", c);
        }
    }

    let (repeat_count, p) = parse_number(&input[p + 1..], p);
    (num_chars, repeat_count, p + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_compress_command() {
        let test1 = "(1x2)";
        let (num_chars, repeat_count, pos) = parse_compress_command(&test1, 1);
        assert_eq!(num_chars, 1);
        assert_eq!(repeat_count, 2);
        assert_eq!(pos, 4);

        let test1 = "(100x2234)";
        let (num_chars, repeat_count, pos) = parse_compress_command(&test1, 1);
        assert_eq!(num_chars, 100);
        assert_eq!(repeat_count, 2234);
        assert_eq!(pos, 9);
    }
}
