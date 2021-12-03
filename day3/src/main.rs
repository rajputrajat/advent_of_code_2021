use std::{fs::read_to_string, str::FromStr};

fn main() {
    let parsed_input = parsing_input_file("day3/input.txt");
    let power_con = get_power_consumption(&parsed_input);
    println!("power consumption is: '{}'", power_con);
}

fn parsing_input_file(path: &str) -> Vec<usize> {
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

struct BinaryNum(usize);

impl FromStr for BinaryNum {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = Self(parse_binary_number_string(s)?);
        Ok(b)
    }
}

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

fn get_power_consumption(input: &[usize]) -> usize {
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
        let bits_count_max = (0_usize | (1 << count_ones_per_bit_pos.len())) - 1;
        let epsilon_rate = bits_count_max ^ gamma_rate;
        epsilon_rate
    };
    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bstr() {
        assert_eq!(parse_binary_number_string("1000").unwrap(), 8);
        assert_eq!(parse_binary_number_string("1100").unwrap(), 12);
    }

    #[test]
    fn to_bits() {
        let mut out = vec![];
        count_bits(0b10110, &mut out);
        assert_eq!(out, vec![0_usize, 1, 1, 0, 1])
    }

    #[test]
    fn power_consumption() {
        let num_input: Vec<usize> = INPUT
            .split_whitespace()
            .map(|s| {
                let b = s.parse::<BinaryNum>().unwrap();
                b.0
            })
            .collect();
        println!("input: {:?}", num_input);
        let pc = get_power_consumption(&num_input);
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
