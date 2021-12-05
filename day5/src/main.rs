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
        if selfd > otherd {
            Some(Ordering::Greater)
        } else if selfd < otherd {
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
                Some(self.cur)
            }
            LineType::Vertical => {
                self.cur.y += 1;
                Some(self.cur)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}
}
