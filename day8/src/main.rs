use std::{fs::read_to_string, str::FromStr};

fn main() {
    let text_input = read_to_string("day8/input.txt").unwrap();
    let input = parse_input(&text_input);
    println!("part1, answer: '{}'", get_identified_digit_count(&input));
}

fn parse_input(input_text: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input_text
        .trim()
        .lines()
        .map(|l| {
            let vstr: Vec<String> = l
                .split_whitespace()
                .filter_map(|s| if s == "|" { None } else { Some(s.to_owned()) })
                .collect();
            assert!(vstr.len() == 14);
            (vstr[0..10].to_owned(), vstr[10..14].to_owned())
        })
        .collect()
}

enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            2 => Ok(Digit::One),
            4 => Ok(Digit::Four),
            3 => Ok(Digit::Seven),
            7 => Ok(Digit::Eight),
            _ => Err(format!("unidentified segment: '{}'", s)),
        }
    }
}

fn get_identified_digit_count(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input
        .iter()
        .map(|(_, v)| {
            let sum_of_digits = v.iter().fold(0, |dig_sum, s| {
                if s.parse::<Digit>().is_ok() {
                    dig_sum + 1
                } else {
                    dig_sum
                }
            });
            sum_of_digits
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{get_identified_digit_count, parse_input};

    #[test]
    fn day8_part1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(26, get_identified_digit_count(&parsed_input));
    }

    const INPUT: &str = r##"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"##;
}
