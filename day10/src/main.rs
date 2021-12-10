use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input_text = read_to_string("day10/input.txt").unwrap();
    println!("part1 answer: {}", find_syntax_error_score(&input_text));
    println!(
        "part2 answer: {}",
        find_middle_scope_incomplete(&input_text)
    );
}

#[derive(Debug, PartialEq)]
enum SymbolType {
    Opening(char),
    Closing(char),
}

fn find_syntax_error_score(input_text: &str) -> usize {
    let mut errors: Vec<char> = Vec::new();
    let pairs = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    for line in input_text.lines() {
        let mut stack: VecDeque<SymbolType> = VecDeque::new();
        'chars: for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => stack.push_back(SymbolType::Opening(c)),
                ']' | ')' | '}' | '>' => {
                    if let Some(popped) = stack.pop_back() {
                        match popped {
                            SymbolType::Opening(p) => {
                                let closing_variant = pairs.get(&p).unwrap();
                                if closing_variant == &c {
                                    continue 'chars;
                                } else {
                                    errors.push(c);
                                    break 'chars;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => unimplemented!(),
            };
        }
    }
    let map: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    errors.iter().fold(0, |acc, c| acc + map.get(c).unwrap())
}

fn find_middle_scope_incomplete(input_text: &str) -> usize {
    let mut errors: Vec<usize> = Vec::new();
    let pairs = HashMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let map: HashMap<char, usize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    'line: for line in input_text.lines() {
        let mut stack: VecDeque<SymbolType> = VecDeque::new();
        'chars: for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => stack.push_back(SymbolType::Opening(c)),
                ']' | ')' | '}' | '>' => {
                    if let Some(popped) = stack.pop_back() {
                        match popped {
                            SymbolType::Opening(p) => {
                                let closing_variant = pairs.get(&p).unwrap();
                                if closing_variant == &c {
                                    continue 'chars;
                                } else {
                                    continue 'line;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => unimplemented!(),
            };
        }
        let score = stack
            .iter_mut()
            .rev()
            .map_while(|sym| match sym {
                SymbolType::Closing(_) => None,
                SymbolType::Opening(sym) => Some(sym),
            })
            .fold(0, |acc, sym| {
                acc * 5 + map.get(pairs.get(sym).unwrap()).unwrap()
            });
        errors.push(score);
    }
    errors.sort();
    errors.iter().nth(errors.len() / 2).copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1() {
        assert_eq!(26397, find_syntax_error_score(INPUT));
    }

    #[test]
    fn day10_part2() {
        assert_eq!(288957, find_middle_scope_incomplete(INPUT));
    }

    const INPUT: &str = r##"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"##;
}
