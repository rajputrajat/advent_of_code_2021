fn main() {}

fn parse_input(input_text: &str) -> Vec<([&str; 10], [&str; 4])> {
    input_text
        .trim()
        .lines()
        .map(|l| {
            let vstr = l
                .split_whitespace()
                .filter_map(|s| if s == "|" { None } else { Some(s) })
                .collect();
            assert_eq!(vstr.len() == 10 + 4);
            (vstr[0..10], vstr[10..14])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn day9_part1() {
        dbg!(parse_input(INPUT));
    }

    const INPUT: &str = r##"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce"##;
}
