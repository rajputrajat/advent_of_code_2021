use std::fs;

fn main() {
    let samples = read_input_file("day1/input.txt");
    let increase_count = part1::OneEntryWindow::how_many_increases(&samples);
    println!(
        "increasing count for single item window: {}",
        increase_count
    );
    let inc_count_3_items = part2::ThreeEntryWindow::how_many_increases(&samples);
    println!(
        "increasing count for three items window: {}",
        inc_count_3_items
    );
}

fn read_input_file(input_file: &str) -> Vec<usize> {
    let text = fs::read_to_string(input_file).unwrap();
    let samples: Vec<usize> = text
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    samples
}

pub(crate) trait Increase {
    fn how_many_increases(samples: &[usize]) -> usize;
}

mod part1 {
    use super::*;

    pub(super) struct OneEntryWindow;
    impl Increase for OneEntryWindow {
        fn how_many_increases(samples: &[usize]) -> usize {
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
    }

    #[test]
    fn sample() {
        let samples = [199_usize, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(OneEntryWindow::how_many_increases(&samples), 7);
    }
}

mod part2 {
    use super::*;
    pub(super) struct ThreeEntryWindow;
    impl Increase for ThreeEntryWindow {
        fn how_many_increases(samples: &[usize]) -> usize {
            let mut count = 0;
            let mut prev = usize::max_value();
            for window in samples.windows(3) {
                let cur: usize = window.iter().sum();
                if cur > prev {
                    count += 1;
                }
                prev = cur;
            }
            count
        }
    }

    #[test]
    fn sample() {
        let samples = [199_usize, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(ThreeEntryWindow::how_many_increases(&samples), 5);
    }
}
