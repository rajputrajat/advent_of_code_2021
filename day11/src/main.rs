use std::fs::read_to_string;

fn main() {
    let mut input = parse_input(&read_to_string("day11/input.txt").unwrap());
    println!("answer of part1 is '{}'", {
        flashes_count_after_100_steps(&mut input.clone())
    });
    println!("answer of part2 is '{}'", {
        step_when_all_flash_at_once(&mut input)
    });
}

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

fn flash(initial_energy_level: &mut Vec<Vec<u8>>, flash_counts: &mut usize) {
    initial_energy_level
        .iter_mut()
        .flatten()
        .for_each(|oct| *oct += 1);
    let mut flash_abs: Vec<(i8, i8)> = Vec::new();
    loop {
        let mut flash_indices: Vec<(i8, i8)> = Vec::new();
        for (il, line) in initial_energy_level.iter().enumerate() {
            for (io, oct) in line.iter().enumerate() {
                if *oct > 9 {
                    flash_indices.push((io as i8, il as i8));
                    *flash_counts += 1;
                }
            }
        }
        flash_abs.extend(&flash_indices);
        if flash_indices.is_empty() {
            break;
        } else {
            flash_indices.iter().for_each(|(i, j)| {
                initial_energy_level[*j as usize][*i as usize] = 0;
                WINDOW
                    .iter()
                    .filter_map(|(i_offset, j_offset)| {
                        let i_added = i + i_offset;
                        let j_added = j + j_offset;
                        if j_added >= 0 && i_added >= 0 && !flash_abs.contains(&(i_added, j_added))
                        {
                            return Some((i_added, j_added));
                        }
                        None
                    })
                    .for_each(|(i, j)| {
                        if let Some(val) = initial_energy_level
                            .get_mut(j as usize)
                            .and_then(|inner| inner.get_mut(i as usize))
                        {
                            *val += 1;
                        }
                    });
            });
        }
        flash_indices.clear();
    }
}

fn flashes_count_after_100_steps(initial_energy_level: &mut Vec<Vec<u8>>) -> usize {
    let mut flash_counts = 0;
    for _ in 0..100 {
        flash(initial_energy_level, &mut flash_counts);
    }
    flash_counts
}

fn step_when_all_flash_at_once(initial_energy_level: &mut Vec<Vec<u8>>) -> usize {
    let mut _flash_counts = 0;
    let mut step_count = 0;
    loop {
        flash(initial_energy_level, &mut _flash_counts);
        step_count += 1;
        if initial_energy_level.iter().flatten().all(|oct| oct == &0) {
            break;
        }
    }
    step_count
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
        let mut input = parse_input(INPUT);
        assert_eq!(1656, flashes_count_after_100_steps(&mut input));
    }

    #[test]
    fn day11_part2() {
        let mut input = parse_input(INPUT);
        assert_eq!(195, step_when_all_flash_at_once(&mut input));
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
