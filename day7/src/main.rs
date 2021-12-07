use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day7/input.txt").unwrap();
    let mut crabs = Crabs::from_text(&input);
    crabs.0.sort();
    println!("part1, answer: '{}'", crabs.get_min(|a| *a));
}

#[derive(Debug)]
struct Crabs(Vec<(u32, u32)>);

impl Crabs {
    fn from_text(input_text: &str) -> Self {
        let mut input: Vec<u32> = input_text
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        input.sort();
        let mut cur = input.first().unwrap();
        let mut cur_count: u32 = 0;
        let mut crabs: Vec<(u32, u32)> = input
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

    fn get_diff_sum<F: Fn(&i32) -> i32>(&self, diff: i32, differ: F) -> u32 {
        //let differ = |d: i32| d * (d + 1) / 2;
        self.0
            .iter()
            .map(|&(num, count)| (num as i32 - differ(&diff)).abs() as u32 * count)
            .sum()
    }

    fn get_min<F: Fn(&i32) -> i32 + Copy>(&self, differ: F) -> u32 {
        let avg = (self.0.iter().map(|(num, count)| num * count).sum::<u32>() / self.0.len() as u32)
            as i32;
        let one_less = self.get_diff_sum(avg - 1, differ);
        let at_avg = self.get_diff_sum(avg, differ);
        let mut min = u32::MAX;
        if one_less < at_avg {
            (0..one_less as u32).into_iter().rev().for_each(|num| {
                let sum = self.get_diff_sum(num as i32, differ);
                if sum < min {
                    min = sum;
                } else {
                    return;
                }
            });
        } else {
            (at_avg as u32..).into_iter().for_each(|num| {
                let sum = self.get_diff_sum(num as i32, differ);
                if sum < min {
                    min = sum;
                } else {
                    return;
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
        let crabs = Crabs::from_text(INPUT);
        assert_eq!(37, crabs.get_min(|a| *a));
    }

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
}
