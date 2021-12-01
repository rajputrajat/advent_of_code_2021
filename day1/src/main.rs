use std::fs;

fn main() {
    let samples = part1::read_input_file("day1/input.txt");
    let increase_count = part1::how_many_increases(samples.iter());
    println!("increasing count: {}", increase_count);
}

mod part1 {
    pub(crate) fn how_many_increases<'a, I: Iterator<Item = &'a usize>>(samples: I) -> usize {
        let mut count = 0;
        let mut prev = usize::max_value();
        for cur in samples {
            if cur > &prev {
                count += 1;
            }
            prev = *cur;
        }
        count
    }

    pub(crate) fn read_input_file<'a>(input_file: &str) -> Vec<usize> {
        let text = super::fs::read_to_string(input_file).unwrap();
        let samples: Vec<usize> = text
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        samples
    }

    #[test]
    fn sample() {
        let samples = [199_usize, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(how_many_increases(samples.iter()), 7);
    }
}

mod part2 {}
