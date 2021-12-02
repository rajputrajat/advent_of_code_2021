use std::{
    fs::File,
    io::{prelude::*, BufReader, Result},
};

fn main() -> Result<()> {
    let reader = BufReader::new(File::open("day2/input.txt")?);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    let product = get_product(&lines);
    println!("product is: {}", product);
    Ok(())
}

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Movement {
    fn parse(line: &str) -> Self {
        let mut split = line.trim().split_whitespace();
        let act = split.next().unwrap();
        let mov: i32 = split.next().unwrap().parse().unwrap();
        match act {
            "forward" => Movement::Forward(mov),
            "down" => Movement::Down(mov),
            "up" => Movement::Up(mov),
            _ => unreachable!(),
        }
    }
}

fn get_product(input: &[String]) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    for line in input {
        let mov = Movement::parse(line);
        match mov {
            Movement::Forward(f) => forward += f,
            Movement::Down(d) => depth += d,
            Movement::Up(u) => depth -= u,
        };
    }
    forward * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_mul() {
        let test_input: Vec<String> = TEST_INPUT.split('\n').map(|s| s.to_owned()).collect();
        assert_eq!(150, get_product(&test_input));
    }

    const TEST_INPUT: &str = r##"forward 5
down 5
forward 8
up 3
down 8
forward 2"##;
}
