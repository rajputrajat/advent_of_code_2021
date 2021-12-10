use std::collections::{HashMap, VecDeque};

fn main() {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1() {
        assert_eq!(26397, find_syntax_error_score(INPUT));
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
