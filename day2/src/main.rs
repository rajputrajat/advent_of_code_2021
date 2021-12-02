fn main() {}

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

fn get_product<'a, I: Iterator<Item = &'a str>>(input: I) -> i32 {
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
        assert_eq!(150, get_product(TEST_INPUT.split('\n')));
    }

    const TEST_INPUT: &str = r##"forward 5
down 5
forward 8
up 3
down 8
forward 2"##;
}
