fn main() {}

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
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

impl Line {
    fn from_points(a: &Point, b: &Point) -> Self {
        Self {
            a: *a,
            b: *b,
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
        if self.typ == LineType::Horizontal && other.typ == LineType::Horizontal {
            vec![]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}
}
