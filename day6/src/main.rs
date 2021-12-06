use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("day6/input.txt").unwrap();
    println!("fish count after 80 days: '{}'", {
        let mut school = NewSchool::from_input(&input_str);
        school.after(80);
        school.count()
    });
    println!("fish count after 256 days: '{}'", {
        let mut school = NewSchool::from_input(&input_str);
        school.after(256);
        school.count()
    });
}

#[derive(Debug)]
struct NewSchool([usize; 10]);

impl NewSchool {
    fn from_input(input_text: &str) -> Self {
        let mut school = NewSchool([0; 10]);
        input_text
            .trim()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .for_each(|n| school.0[n] += 1);
        school
    }

    fn after(&mut self, days: usize) {
        (0..days - 1).into_iter().for_each(|_| {
            self.pass_a_day();
        });
    }

    fn pass_a_day(&mut self) {
        self.0.rotate_left(1);
        if self.0[0] > 0 {
            self.0[7] += self.0[0];
            self.0[9] += self.0[0];
            self.0[0] = 0;
        }
    }

    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_18_days() {
        let mut school = NewSchool::from_input(INPUT);
        school.after(18);
        assert_eq!(26, school.count());
    }

    #[test]
    fn after_80_days() {
        let mut school = NewSchool::from_input(INPUT);
        school.after(80);
        assert_eq!(5934, school.count());
    }

    #[test]
    fn after_256_days() {
        let mut school = NewSchool::from_input(INPUT);
        school.after(256);
        assert_eq!(26984457539, school.count());
    }

    const INPUT: &str = r"3,4,3,1,2";
}
