fn main() {
    println!("Hello, world!");
}

fn how_many_increases<'a, I: Iterator<Item = &'a usize>>(samples: I) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let samples = [199_usize, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(how_many_increases(samples.iter()), 7);
    }
}
