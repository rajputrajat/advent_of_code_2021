use std::fs::read_to_string;

fn main() {
    let text_input = read_to_string("day9/input.txt").unwrap();
    let mut height_map = HeightMap::parse_input(&text_input);
    let sum = height_map.get_sum_lowest_points();
    println!("part1 result: '{}'", sum);
}

#[derive(Debug, Clone, Copy)]
struct Index {
    x: isize,
    y: isize,
}

impl Index {
    fn up(&self) -> Self {
        let mut i = *self;
        i.y -= 1;
        i
    }
    fn down(&self) -> Self {
        let mut i = *self;
        i.y += 1;
        i
    }
    fn left(&self) -> Self {
        let mut i = *self;
        i.x -= 1;
        i
    }
    fn right(&self) -> Self {
        let mut i = *self;
        i.x += 1;
        i
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Location {
    Centre,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    digit: u8,
    index: Index,
    lowest: Option<Location>,
}

impl Point {
    fn up(&self, map: &HeightMap) -> Option<Self> {
        map.get_point(self.index.up()).copied()
    }
    fn down(&self, map: &HeightMap) -> Option<Self> {
        map.get_point(self.index.down()).copied()
    }
    fn left(&self, map: &HeightMap) -> Option<Self> {
        map.get_point(self.index.left()).copied()
    }
    fn right(&self, map: &HeightMap) -> Option<Self> {
        map.get_point(self.index.right()).copied()
    }
    fn set_adjacent_lowest(&mut self, map: &HeightMap) {
        let mut lowest = (Location::Centre, self.digit);
        if let Some(u) = self.up(map) {
            if u.digit < self.digit {
                lowest = (Location::Up, u.digit);
            }
        }
        if let Some(d) = self.down(map) {
            if d.digit < lowest.1 {
                lowest = (Location::Down, d.digit);
            }
        }
        if let Some(l) = self.left(map) {
            if l.digit < lowest.1 {
                lowest = (Location::Left, l.digit);
            }
        }
        if let Some(r) = self.right(map) {
            if r.digit < lowest.1 {
                lowest = (Location::Right, r.digit);
            }
        }
        self.lowest = Some(lowest.0);
    }
}

#[derive(Debug, Clone)]
struct HeightMap {
    points: Vec<Point>,
    width: isize,
    height: isize,
}

impl HeightMap {
    fn parse_input(input_text: &str) -> Self {
        let nums: Vec<Vec<Point>> = input_text
            .trim()
            .lines()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| Point {
                        digit: c.to_string().parse().unwrap(),
                        index: Index {
                            x: x as isize,
                            y: y as isize,
                        },
                        lowest: None,
                    })
                    .collect()
            })
            .collect();
        let height = nums.len() as isize;
        let width = nums.first().unwrap().len() as isize;
        Self {
            points: nums.iter().flatten().map(|p| p.clone()).collect(),
            width,
            height,
        }
    }

    fn get_point(&self, index: Index) -> Option<&Point> {
        let index = index.y * self.width + index.x;
        self.points.get(index as usize)
    }

    fn get_lowest_points(&mut self) -> Vec<Point> {
        let map_clone = self.clone();
        self.points
            .iter_mut()
            .for_each(|p| p.set_adjacent_lowest(&map_clone));
        self.points
            .iter()
            .filter_map(|p| {
                if p.lowest.unwrap() == Location::Centre {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_sum_lowest_points(&mut self) -> usize {
        let sum = self
            .get_lowest_points()
            .iter()
            .fold(0, |acc, p| acc + p.digit as usize + 1);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1() {
        let mut map = HeightMap::parse_input(INPUT);
        assert_eq!(15, map.get_sum_lowest_points());
    }

    const INPUT: &str = r##"2199943210
3987894921
9856789892
8767896789
9899965678"##;
}
