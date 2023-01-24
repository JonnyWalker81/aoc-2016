type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let input = "cxdnnyjw";
    // let input = "abc";
    part1(input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut i = 0;

    let mut password = String::new();
    for _ in 0..8 {
        let (hash, index) = next_hash(input, i);
        i = index + 1;
        let c = hash.chars().nth(5).unwrap();
        password.push(c);
    }

    println!("{password}");

    Ok(())
}

fn next_hash(input: &str, mut i: usize) -> (String, usize) {
    loop {
        let n = format!("{}{}", input, i);
        let md5_hash = md5::compute(n);
        let index = format!("{:x}", md5_hash);
        if index.starts_with("00000") {
            return (index, i);
        }

        i += 1;
    }
}

fn part2(input: &str) -> Result<()> {
    let mut i = 0;

    let mut password = vec!['\0'; 8];
    let mut count = 0;
    loop {
        let (hash, index) = next_hash(input, i);
        i = index + 1;
        let pc = hash.chars().nth(5).unwrap();
        let c = hash.chars().nth(6).unwrap();
        let p = usize::from_str_radix(&pc.to_string(), 16).unwrap();
        if p >= 8 || password[p] != '\0' {
            continue;
        }

        password[p] = c;
        count += 1;
        if count >= 8 {
            break;
        }
    }

    let pass: String = password.iter().collect();
    println!("{pass}");

    Ok(())
}
