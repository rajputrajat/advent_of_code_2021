use std::{
    cmp::{Ordering, PartialOrd},
    fs::read_to_string,
};

fn main() {
    let text = read_to_string("day5/input.txt").unwrap();
    let lines = parse_input(&text);
    //let crosses_count = get_crossing_points_count(&lines);
    //println!("crosses count: '{}'", crosses_count);
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self) -> f32 {
        let sq_sum = self.x.pow(2) + self.y.pow(2);
        (sq_sum as f32).sqrt()
    }

    fn farthest_dimension(&self) -> isize {
        if self.x > self.y {
            self.x + 1
        } else {
            self.y + 1
        }
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
pub enum LineType {
    Horizontal,
    Vertical,
    At45,
    Angled,
}

#[derive(Debug)]
pub struct Line {
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
        let mut ret_val = None;
        match self.line.typ {
            LineType::Horizontal => {
                if self.cur.x <= self.line.b.x {
                    ret_val = Some(self.cur);
                }
                self.cur.x += 1;
            }
            LineType::Vertical => {
                if self.cur.y <= self.line.b.y {
                    ret_val = Some(self.cur);
                }
                self.cur.y += 1;
            }
            LineType::At45 => {
                if self.cur.x == self.line.b.x && self.cur.y == self.line.b.y {
                    ret_val = Some(self.cur);
                }
                if self.line.b.y > self.line.a.y {
                    self.cur.y += 1;
                } else {
                    self.cur.y -= 1;
                }
                if self.line.b.x > self.line.a.x {
                    self.cur.x += 1;
                } else {
                    self.cur.x -= 1;
                }
            }
            LineType::Angled => unimplemented!(),
        };
        return ret_val;
    }
}

type FillCount = isize;

struct Canvas {
    data: Vec<FillCount>,
    side: usize,
}

struct Row<'r> {
    canvas: &'r Canvas,
    row_index: usize,
    column_index: usize,
}

impl<'r> Iterator for Row<'r> {
    type Item = FillCount;
    fn next(&mut self) -> Option<Self::Item> {
        let p = self
            .canvas
            .data
            .get(self.column_index * self.canvas.side + self.row_index);
        self.column_index += 1;
        p.copied()
    }
}

struct Column<'c> {
    canvas: &'c Canvas,
    row_index: usize,
    column_index: usize,
}

impl<'c> Iterator for Column<'c> {
    type Item = FillCount;
    fn next(&mut self) -> Option<Self::Item> {
        let p = self
            .canvas
            .data
            .get(self.column_index * self.canvas.side + self.row_index);
        self.row_index += 1;
        p.copied()
    }
}

impl Canvas {
    fn mark_line(&mut self, line: &Line) {
        line.points()
            .inspect(|p| {
                if line.typ == LineType::At45 {
                    println!("{:?}", p)
                }
            })
            .for_each(|p| self.mark_point(&p));
    }

    fn mark_point(&mut self, point: &Point) {
        let index = point.x + self.side as isize * point.y;
        self.data[index as usize] += 1;
    }

    fn row(&self, row_index: usize) -> Row {
        Row {
            canvas: self,
            row_index,
            column_index: 0,
        }
    }

    fn column(&self, column_index: usize) -> Column {
        Column {
            canvas: self,
            row_index: 0,
            column_index,
        }
    }

    fn from_lines(lines: &[Line]) -> Self {
        let side = lines.iter().fold(0, |max, l| {
            let lf = l.farthest_dimension();
            if max > lf {
                max
            } else {
                lf
            }
        }) as usize;
        let data: Vec<FillCount> = std::iter::repeat(0).take(side.pow(2)).collect();
        Self { data, side }
    }

    fn marks_count_larger(&self, than: FillCount) -> usize {
        self.data
            .iter()
            .fold(0, |count, p| if *p > than { count + 1 } else { count })
    }
}

impl Line {
    fn points(&self) -> PointIter {
        PointIter {
            line: self,
            cur: self.a,
        }
    }

    pub fn from_points(a: &Point, b: &Point) -> Self {
        let (larger, smaller) = if a > b { (a, b) } else { (b, a) };
        Self {
            a: *larger,
            b: *smaller,
            typ: if a.x == b.x {
                LineType::Vertical
            } else if a.y == b.y {
                LineType::Horizontal
            } else if (a.x - b.x).abs() == (a.y - b.y).abs() {
                LineType::At45
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
                .filter_map(|sp| other.points().find(|op| op == &sp))
                .collect::<Vec<Point>>()
        }
    }

    fn farthest_dimension(&self) -> isize {
        let a = self.a.farthest_dimension();
        let b = self.b.farthest_dimension();
        if a > b {
            a
        } else {
            b
        }
    }
}

fn parse_input(input_text: &str) -> Vec<Line> {
    input_text
        .split('\n')
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let nums: Vec<isize> = l
                    .split(&[' ', ',', '-', '>'][..])
                    .filter_map(|s| s.to_string().parse().ok())
                    .collect();
                debug_assert_eq!(nums.len(), 4);
                Some(Line::from_points(
                    &Point {
                        x: nums[0],
                        y: nums[1],
                    },
                    &Point {
                        x: nums[2],
                        y: nums[3],
                    },
                ))
            }
        })
        .collect()
}

pub fn _process_all_nodes(lines: &[Line]) -> Vec<Point> {
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

pub fn get_crossing_points_count(lines: &[Line], ignore: &[LineType]) -> usize {
    let mut canvas = Canvas::from_lines(lines);
    for line in lines.iter().filter(|l| !ignore.contains(&l.typ)) {
        dbg!(line);
        canvas.mark_line(line);
    }
    let crossed_points = canvas.marks_count_larger(1);
    crossed_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_line_crosses() {
        let input = parse_input(INPUT);
        let crossing_points =
            get_crossing_points_count(&input, &[LineType::At45, LineType::Angled]);
        assert_eq!(5, crossing_points);
    }

    #[test]
    fn part2_line_crosses() {
        let input = parse_input(INPUT);
        let crossing_points = get_crossing_points_count(&input, &[LineType::Angled]);
        assert_eq!(12, crossing_points);
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
