fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Index {
    x: i16,
    y: i16,
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
    width: i16,
    height: i16,
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
                            x: x as i16,
                            y: y as i16,
                        },
                        lowest: None,
                    })
                    .collect()
            })
            .collect();
        let height = nums.len() as i16;
        let width = nums.first().unwrap().len() as i16;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1() {
        let mut map = HeightMap::parse_input(INPUT);
        let risk_levels = map
            .get_lowest_points()
            .iter()
            .fold(0, |acc, p| acc + p.digit + 1);
        assert_eq!(15, risk_levels);
    }

    const INPUT: &str = r##"2199943210
3987894921
9856789892
8767896789
9899965678"##;
}