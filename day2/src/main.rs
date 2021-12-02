use std::{
    fs::File,
    io::{prelude::*, BufReader, Result},
};

fn main() -> Result<()> {
    let reader = BufReader::new(File::open("day2/input.txt")?);
    let input: Vec<_> = reader.lines().map(|s| s.unwrap()).collect();
    let moves = parse_input(&input);
    let product_part1 = part1::get_product(&moves);
    println!("product is: {}", product_part1);
    let improved_product = part2::get_product(&moves);
    println!("improve product is '{}'", improved_product);
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

fn parse_input(input: &[String]) -> Vec<Movement> {
    input.iter().map(|s| Movement::parse(&s)).collect()
}

mod part1 {
    use super::*;
    pub(super) fn get_product(moves: &[Movement]) -> i32 {
        let mut forward = 0;
        let mut depth = 0;
        for m in moves {
            match m {
                Movement::Forward(f) => forward += f,
                Movement::Down(d) => depth += d,
                Movement::Up(u) => depth -= u,
            };
        }
        forward * depth
    }

    #[test]
    fn pos_mul() {
        let test_input: Vec<String> = TEST_INPUT.split('\n').map(|s| s.to_owned()).collect();
        let moves = parse_input(&test_input);
        assert_eq!(150, get_product(&moves));
    }
}

mod part2 {
    use super::*;
    pub(super) fn get_product(moves: &[Movement]) -> i32 {
        let mut forward = 0;
        let mut depth = 0;
        let mut aim = 0;
        for m in moves {
            match m {
                Movement::Forward(f) => {
                    forward += f;
                    depth += aim * f;
                }
                Movement::Down(d) => aim += d,
                Movement::Up(u) => aim -= u,
            };
        }
        forward * depth
    }

    #[test]
    fn pos_mul() {
        let test_input: Vec<String> = TEST_INPUT.split('\n').map(|s| s.to_owned()).collect();
        let moves = parse_input(&test_input);
        assert_eq!(900, get_product(&moves));
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r##"forward 5
down 5
forward 8
up 3
down 8
forward 2"##;
