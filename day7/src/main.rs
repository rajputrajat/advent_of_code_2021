use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day7/input.txt").unwrap();
    let mut crabs = Crabs::from_text(&input);
    crabs.0.sort();
    let mut min_diff_sum = isize::MIN;
    // println!("{:?}", parsed);
    // let avg = parsed.iter().sum::<isize>() / parsed.len() as isize;
    // (0..200).into_iter().for_each(|n| {
    //     let num = avg - n;
    //     println!("for '{}': {}", &num, get_diff_sum(num, &parsed));
    // });
    // println!("{}, {}", parsed.len(), parsed.iter().sum::<isize>());
}

struct Crabs(Vec<isize>);

impl Crabs {
    fn from_text(input_text: &str) -> Self {
        Self(
            input_text
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }

    fn get_diff_sum(&self, diff: isize) -> isize {
        self.0.iter().map(|&n| (n - diff).abs()).sum()
    }

    fn get_min(&self) -> isize {
        let avg = self.0.iter().sum::<isize>() / self.0.len() as isize;
        let one_less = self.get_diff_sum(avg - 1);
        let at_avg = self.get_diff_sum(avg);
        let mut min = isize::MAX;
        if one_less < at_avg {
            (0..one_less as usize).into_iter().rev().for_each(|num| {
                let sum = self.get_diff_sum(num as isize);
                if sum < min {
                    min = sum;
                }
            });
        } else {
            (at_avg as usize..).into_iter().for_each(|num| {
                let sum = self.get_diff_sum(num as isize);
                if sum < min {
                    min = sum;
                }
            });
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1() {
        let mut crabs = Crabs::from_text(INPUT);
        crabs.0.sort();
        assert_eq!(37, crabs.get_min());
    }

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
}
