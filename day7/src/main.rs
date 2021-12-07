use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day7/input.txt").unwrap();
    let crabs = Crabs::from_text(&input);
    println!("part1, answer: '{}'", crabs.get_min(|a| *a));
    println!("part2, answer: '{}'", crabs.get_min(|&a| a * (a + 1) / 2));
}

#[derive(Debug)]
struct Crabs(Vec<(usize, usize)>);

impl Crabs {
    fn from_text(input_text: &str) -> Self {
        let mut input: Vec<usize> = input_text
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        input.sort();
        let mut cur = input.first().unwrap();
        let mut cur_count: usize = 0;
        let mut crabs: Vec<(usize, usize)> = input
            .iter()
            .filter_map(|n| {
                if n == cur {
                    cur_count += 1;
                    None
                } else {
                    let ret = (*cur, cur_count);
                    cur_count = 1;
                    cur = n;
                    Some(ret)
                }
            })
            .collect();
        crabs.push((*cur, cur_count));
        Self(crabs)
    }

    fn get_diff_sum<F: Fn(&isize) -> isize>(&self, diff: isize, differ: F) -> usize {
        self.0
            .iter()
            .map(|&(num, count)| differ(&(num as isize - diff).abs()) as usize * count)
            .sum()
    }

    fn get_min<F: Fn(&isize) -> isize + Copy>(&self, differ: F) -> usize {
        let mut min = usize::MAX;
        (0..self.0.len()).into_iter().for_each(|num| {
            let sum = self.get_diff_sum(num as isize, differ);
            if sum < min {
                min = sum;
            } else {
                return;
            }
        });
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1() {
        let crabs = Crabs::from_text(INPUT);
        assert_eq!(37, crabs.get_min(|a| *a));
    }

    #[test]
    fn day7_part2() {
        let crabs = Crabs::from_text(INPUT);
        assert_eq!(168, crabs.get_min(|&a| a * (a + 1) / 2));
    }

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
}
