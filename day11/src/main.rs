fn main() {}

const WINDOW: &[(i8, i8); 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn flashes_count_after_100_steps(mut initial_energy_level: Vec<Vec<u8>>) -> usize {
    let mut flash_counts = 0;
    for _ in 0..2 {
        let mut flash_indices: Vec<(i8, i8)> = Vec::new();
        for line in initial_energy_level.iter_mut() {
            for oct in line.iter_mut() {
                *oct += 1;
            }
        }
        loop {
            for (il, line) in initial_energy_level.iter().enumerate() {
                for (io, oct) in line.iter().enumerate() {
                    if *oct > 9 {
                        flash_indices.push((io as i8, il as i8));
                        flash_counts += 1;
                    }
                }
            }
            println!("{:?}", flash_indices);
            if flash_indices.is_empty() {
                break;
            } else {
                flash_indices.iter().for_each(|(i, j)| {
                    WINDOW.iter().for_each(|(ioff, joff)| {
                        let i_added = i + ioff;
                        let j_added = j + joff;
                        if j_added >= 0 && i_added >= 0 {
                            if let Some(val) = initial_energy_level
                                .get_mut(j_added as usize)
                                .and_then(|inner| inner.get_mut(i_added as usize))
                            {
                                *val += 1;
                            }
                        }
                    });
                    initial_energy_level[*j as usize][*i as usize] = 0;
                });
            }
            flash_indices.clear();
        }
        println!("{:?}", initial_energy_level);
    }
    flash_counts
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("not a digit") as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1() {
        let input = parse_input(INPUT);
        assert_eq!(1656, flashes_count_after_100_steps(input));
    }

    const INPUT: &str = r##"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"##;
}
