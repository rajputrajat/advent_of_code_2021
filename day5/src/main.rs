use std::cmp::{Ordering, PartialOrd};

fn main() {}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn distance(&self) -> f32 {
        let sq_sum = self.x.pow(2) + self.y.pow(2);
        (sq_sum as f32).sqrt()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let selfd = self.distance();
        let otherd = other.distance();
        if selfd < otherd {
            Some(Ordering::Greater)
        } else if selfd > otherd {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug, PartialEq)]
enum LineType {
    Horizontal,
    Vertical,
    Angled,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
    typ: LineType,
}

struct PointIter<'l> {
    line: &'l Line,
    cur: Point,
}

impl<'l> Iterator for PointIter<'l> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        match self.line.typ {
            LineType::Horizontal => {
                self.cur.x += 1;
                if self.cur.x > self.line.b.x {
                    None
                } else {
                    Some(self.cur)
                }
            }
            LineType::Vertical => {
                self.cur.y += 1;
                if self.cur.y > self.line.b.y {
                    None
                } else {
                    Some(self.cur)
                }
            }
            LineType::Angled => unimplemented!(),
        }
    }
}

impl Line {
    fn points(&self) -> PointIter {
        PointIter {
            line: self,
            cur: self.a,
        }
    }

    fn from_points(a: &Point, b: &Point) -> Self {
        let (larger, smaller) = if a > b { (a, b) } else { (b, a) };
        Self {
            a: *larger,
            b: *smaller,
            typ: if a.x == b.x {
                LineType::Horizontal
            } else if a.y == b.y {
                LineType::Vertical
            } else {
                LineType::Angled
            },
        }
    }

    fn crosses(&self, other: &Line) -> Vec<Point> {
        if self.typ == LineType::Angled || other.typ == LineType::Angled {
            vec![]
        } else {
            self.points()
                .filter_map(|sp| {
                    if other.points().any(|op| op == sp) {
                        Some(sp)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        }
    }
}

fn parse_input(input_text: &str) -> Vec<Line> {
    input_text
        .split('\n')
        .map(|l| {
            let nums: Vec<u16> = l
                .chars()
                .filter_map(|c| {
                    if c == ' ' || c == ',' || c == '-' || c == '>' {
                        None
                    } else {
                        Some(c.to_string().parse::<u16>().unwrap())
                    }
                })
                .collect();
            debug_assert_eq!(nums.len(), 4);
            Line::from_points(
                &Point {
                    x: nums[0],
                    y: nums[1],
                },
                &Point {
                    x: nums[2],
                    y: nums[3],
                },
            )
        })
        .collect()
}

fn process_all_nodes(lines: &[Line]) -> Vec<Point> {
    let mut current_index = 1;
    let mut crossing_points: Vec<Point> = vec![];
    loop {
        let (first_node, remaining) = lines.split_at(current_index);
        let first_node = first_node.iter().last().unwrap();
        let crosses: Vec<Point> = remaining
            .iter()
            .map(|l| l.crosses(&first_node))
            .flatten()
            .collect();
        crossing_points.extend_from_slice(&crosses);
        current_index += 1;
        if current_index == lines.len() {
            break;
        }
    }
    crossing_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = parse_input(INPUT);
        let crossing_points = process_all_nodes(&input);
        assert_eq!(5, crossing_points.len());
    }

    const INPUT: &str = r##"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"##;
}
