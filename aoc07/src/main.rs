use std::{
    collections::{HashMap, HashSet},
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
    let addresses: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut valid = 0;
    for a in addresses {
        if is_valid_tls_address(a.as_slice()) {
            valid += 1;
        }
    }

    println!("Valid: {valid}");

    Ok(())
}

fn is_valid_tls_address(address: &[char]) -> bool {
    let mut found_abba = false;
    let mut inside_brackets = false;
    for w in address.windows(4) {
        if w[0] == w[3] && w[1] == w[2] && w[1] != w[0] {
            if !inside_brackets {
                found_abba = true;
            } else {
                return false;
            }
        }

        if w[0] == '[' {
            inside_brackets = true;
        } else if w[0] == ']' {
            inside_brackets = false;
        }
    }

    found_abba
}

fn part2(input: &str) -> Result<()> {
    let addresses: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut valid = 0;
    for a in addresses {
        if is_valid_ssl_address(a.as_slice()) {
            valid += 1;
        }
    }

    println!("Valid: {valid}");

    Ok(())
}

fn is_valid_ssl_address(address: &[char]) -> bool {
    let mut found_aba = HashSet::new();
    let mut inside_brackets = false;
    let mut found_bab = HashSet::new();
    for w in address.windows(3) {
        if w[0] == w[2] && w[1] != w[0] {
            let key: String = w.iter().map(|c| c).collect();
            if !inside_brackets {
                found_aba.insert(key);
            } else {
                found_bab.insert(key);
            }
        }

        if w[0] == '[' {
            inside_brackets = true;
        } else if w[0] == ']' {
            inside_brackets = false;
        }
    }

    for aba in found_aba {
        let cs: Vec<char> = aba.chars().collect();
        let mut bab = vec!['\0'; cs.len()];
        bab[0] = cs[1];
        bab[1] = cs[0];
        bab[2] = cs[1];

        let key: String = bab.iter().collect();
        if found_bab.contains(&key) {
            return true;
        }
    }

    false
}
