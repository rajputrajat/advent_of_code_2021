use std::{fs::read_to_string, str::FromStr};

fn main() {
    println!("power consumption is: '{}'", {
        let parsed_input = part1::parsing_input_file("day3/input.txt");
        part1::get_power_consumption(&parsed_input)
    });
    println!("life support rating: '{}'", {
        let part2_input = part2::parsing_input_file("day3/input.txt");
        part2::get_life_support_rating(part2::parse_as_num_vec(&part2_input))
    });
}

mod part1 {
    use super::*;

    pub(super) fn parsing_input_file(path: &str) -> Vec<usize> {
        let text = read_to_string(path).unwrap();
        text.split_whitespace()
            .map(|s| {
                let b = s.parse::<BinaryNum>().unwrap();
                b.0
            })
            .collect()
    }

    fn count_bits(input: usize, out_count: &mut Vec<usize>) {
        let mut bit_pos = 0;
        let mut i = input;
        loop {
            if i == 0 {
                break;
            }
            let value_at_current_bit_pos = if (i & 0x1) != 0 { 1 } else { 0 };
            if bit_pos + 1 > out_count.len() {
                out_count.push(value_at_current_bit_pos);
            } else {
                out_count[bit_pos] += value_at_current_bit_pos;
            }
            bit_pos += 1;
            i >>= 1;
        }
    }

    pub(super) struct BinaryNum(pub(super) usize);

    impl FromStr for BinaryNum {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let b = Self(BinaryNum::parse_binary_number_string(s)?);
            Ok(b)
        }
    }

    impl BinaryNum {
        fn parse_binary_number_string(input_str: &str) -> Result<usize, String> {
            let mut num = 0;
            for c in input_str.chars() {
                let c = c.to_string();
                num <<= 1;
                num |= c
                    .parse::<usize>()
                    .map_err(|e| format!("could not parse. e: '{}'", e))?;
            }
            Ok(num)
        }
    }

    pub(super) fn get_power_consumption(input: &[usize]) -> usize {
        let mut count_ones_per_bit_pos = Vec::<usize>::new();
        for i in input {
            count_bits(*i, &mut count_ones_per_bit_pos);
        }
        let half_of_max_ones_or_zeros = input.len() / 2;
        let gamma_rate = {
            let mut gamma_rate = 0;
            for (i, count_for_bit) in count_ones_per_bit_pos.iter().enumerate() {
                if count_for_bit >= &half_of_max_ones_or_zeros {
                    gamma_rate |= 1 << i;
                }
            }
            gamma_rate
        };
        let epsilon_rate = {
            let bits_count_max = (1 << count_ones_per_bit_pos.len()) - 1;
            bits_count_max ^ gamma_rate
        };
        gamma_rate * epsilon_rate
    }

    #[test]
    fn parse_bstr() {
        assert_eq!(BinaryNum::parse_binary_number_string("1000").unwrap(), 8);
        assert_eq!(BinaryNum::parse_binary_number_string("1100").unwrap(), 12);
    }

    #[test]
    fn to_bits() {
        let mut out = vec![];
        count_bits(0b10110, &mut out);
        assert_eq!(out, vec![0_usize, 1, 1, 0, 1])
    }
}

mod part2 {
    use super::*;
    pub(super) fn parsing_input_file(path: &str) -> Vec<String> {
        let text = read_to_string(path).unwrap();
        text.split_whitespace().map(|s| s.to_string()).collect()
    }

    pub(super) fn parse_as_num_vec(input_str: &[String]) -> Vec<Vec<u8>> {
        let vv_nums: Vec<Vec<u8>> = input_str
            .iter()
            .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        vv_nums
    }

    pub(super) fn get_life_support_rating(vec_nums: Vec<Vec<u8>>) -> usize {
        let o2_rating = {
            let mut working_indices: Vec<usize> =
                vec_nums.iter().enumerate().map(|(i, _)| i).collect();
            let mut bit_pos = 0;
            loop {
                let mut more_ones = (0, vec![]);
                let mut more_zeroes = more_ones.clone();
                working_indices.iter().for_each(|&i| {
                    if vec_nums[i][bit_pos] == 1 {
                        more_ones.0 += 1;
                        more_ones.1.push(i);
                    } else {
                        more_zeroes.0 += 1;
                        more_zeroes.1.push(i);
                    }
                });
                working_indices = if more_ones.0 >= more_zeroes.0 {
                    more_ones.1
                } else {
                    more_zeroes.1
                };
                bit_pos += 1;
                if working_indices.len() == 1 {
                    break;
                }
            }
            binary_vec_to_num(&vec_nums[working_indices[0]])
        };

        let co2_rating = {
            let mut working_indices: Vec<usize> =
                vec_nums.iter().enumerate().map(|(i, _)| i).collect();
            let mut bit_pos = 0;
            loop {
                let mut more_ones = (0, vec![]);
                let mut more_zeroes = more_ones.clone();
                working_indices.iter().for_each(|&i| {
                    if vec_nums[i][bit_pos] == 1 {
                        more_ones.0 += 1;
                        more_ones.1.push(i);
                    } else {
                        more_zeroes.0 += 1;
                        more_zeroes.1.push(i);
                    }
                });
                working_indices = if more_ones.0 >= more_zeroes.0 {
                    more_zeroes.1
                } else {
                    more_ones.1
                };
                bit_pos += 1;
                if working_indices.len() == 1 {
                    break;
                }
            }
            binary_vec_to_num(&vec_nums[working_indices[0]])
        };
        o2_rating * co2_rating
    }

    fn binary_vec_to_num(bin_vec: &[u8]) -> usize {
        bin_vec
            .iter()
            .rev()
            .enumerate()
            .fold(0_usize, |acc, (i, &x)| acc | (x as usize) << i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn vec_to_num() {
            assert_eq!(binary_vec_to_num(&[1, 0, 0, 1, 0]), 18);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_str_to_num_vecs() {
        assert_eq!(
            part2::parse_as_num_vec(&["11000".to_owned(), "10010".to_owned()]),
            vec![vec![1, 1, 0, 0, 0], vec![1, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn life_support_rating() {
        let num_input: Vec<String> = INPUT.split_whitespace().map(|s| s.to_owned()).collect();
        let nvec = part2::parse_as_num_vec(&num_input);
        assert_eq!(230, part2::get_life_support_rating(nvec));
    }

    #[test]
    fn power_consumption() {
        let num_input: Vec<usize> = INPUT
            .split_whitespace()
            .map(|s| {
                let b = s.parse::<part1::BinaryNum>().unwrap();
                b.0
            })
            .collect();
        println!("input: {:?}", num_input);
        let pc = part1::get_power_consumption(&num_input);
        assert_eq!(198, pc);
    }

    const INPUT: &str = r##"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"##;
}
