fn main() {
    println!("Hello, world!");
}

pub fn how_many_increases<'a, I: Iterator<Item = &'a usize>>(samples: I) -> usize {
    let increased = |a: &usize, b: &usize| b > a;
    let mut count = 0;
    let mut prev = usize::max_value();
    for cur in samples {
        if true == increased(&prev, cur) {
            count += 1;
        }
        prev = *cur;
    }
    count
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
