use anyhow::Result;

fn main() {}

#[derive(Clone, Copy, PartialEq)]
enum DaubStatus {
    NotMarked,
    Marked,
}

pub enum WinStatus {
    Pass,
    Win,
}

pub struct BallCall(u8);

trait Bingo {
    fn next_ball_call() -> BallCall;
}

#[derive(Clone, Copy)]
struct Cell(u8, DaubStatus);

impl Cell {
    fn from_num(cell_num: u8) -> Self {
        Self(cell_num, DaubStatus::NotMarked)
    }
}

pub struct BingoCard(Vec<Cell>);

impl BingoCard {
    const ROW_COUNT: u8 = 5;
    const COLUMN_COUNT: u8 = 5;

    pub fn apply_ball_call(&mut self, ball: &BallCall) -> WinStatus {
        if let Some(found_cell) = self.0.iter_mut().find(|c| c.0 == ball.0) {
            found_cell.1 = DaubStatus::Marked;
            return self.check_win();
        } else {
            return WinStatus::Pass;
        }
    }

    fn check_win(&self) -> WinStatus {
        let win = (0..BingoCard::ROW_COUNT).any(|r| self.row(r).all(|c| c.1 == DaubStatus::Marked))
            || (0..BingoCard::COLUMN_COUNT)
                .any(|c| self.column(c).all(|r| r.1 == DaubStatus::Marked));
        if win {
            WinStatus::Win
        } else {
            WinStatus::Pass
        }
    }

    fn row(&self, row_index: u8) -> Row {
        Row {
            card: self,
            row_index,
            column_index: 0,
        }
    }

    fn column(&self, column_index: u8) -> Column {
        Column {
            card: self,
            row_index: 0,
            column_index,
        }
    }
}

struct Row<'b> {
    card: &'b BingoCard,
    row_index: u8,
    column_index: u8,
}

impl<'b> Iterator for Row<'b> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.column_index == BingoCard::COLUMN_COUNT {
            None
        } else {
            let index = self.column_index + (5 * self.row_index);
            let rv = self.card.0.get(index as usize);
            self.column_index += 1;
            rv.map(|x| *x)
        }
    }
}

struct Column<'b> {
    card: &'b BingoCard,
    column_index: u8,
    row_index: u8,
}

impl<'b> Iterator for Column<'b> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.row_index == BingoCard::ROW_COUNT {
            None
        } else {
            let index = self.column_index + (5 * self.row_index);
            let rv = self.card.0.get(index as usize);
            self.row_index += 1;
            rv.map(|x| *x)
        }
    }
}

struct BallCalls(Vec<BallCall>);

struct BingoData {
    ball_calls: BallCalls,
    cards: Vec<BingoCard>,
}

fn parse_input(input: &str) -> BingoData {
    let mut iter = input.split_whitespace();
    let ballcalls_str = iter.next().expect("first csv are missing");
    let bingo_cards_str: Vec<&str> = iter.collect();
    let ball_calls: Vec<BallCall> = ballcalls_str
        .split(',')
        .map(|c| BallCall(c.parse().expect("invalid char to parse")))
        .collect();
    let ball_calls = BallCalls(ball_calls);
    let cards: Vec<BingoCard> = bingo_cards_str
        .chunks_exact(5 * 5)
        .map(|card_str| {
            let cell_vec: Vec<Cell> = card_str
                .iter()
                .map(|c| {
                    let cell_num = c.parse::<u8>().unwrap();
                    Cell::from_num(cell_num)
                })
                .collect();
            BingoCard(cell_vec)
        })
        .collect();
    BingoData { ball_calls, cards }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_balls() {
        let data = parse_input(INPUT);
        let first_5: Vec<u8> = data.ball_calls.0.iter().take(5).map(|b| b.0).collect();
        let last_5: Vec<u8> = data
            .ball_calls
            .0
            .iter()
            .rev()
            .take(5)
            .map(|b| b.0)
            .collect();
        assert_eq!(vec![7, 4, 9, 5, 11], first_5);
        assert_eq!(vec![1, 26, 3, 19, 8], last_5);
    }

    #[test]
    fn fist_card_2nd_row() {
        let data = parse_input(INPUT);
        let first_card = data.cards.get(0).unwrap();
        let second_row: Vec<u8> = first_card.row(1).map(|c| c.0).collect();
        assert_eq!(vec![8, 2, 23, 4, 24], second_row);
    }

    #[test]
    fn test_sample_input() {}

    const INPUT: &str = r##"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"##;
}
