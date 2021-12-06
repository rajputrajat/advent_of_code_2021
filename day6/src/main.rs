fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct LanternFish {
    days_untill_repro: usize,
    status: FishStatus,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum FishStatus {
    NewBorn,
    RecentlyGaveBirth,
    Normal,
}

impl LanternFish {
    fn from_until_days(until: usize) -> Self {
        Self {
            days_untill_repro: until,
            status: FishStatus::Normal,
        }
    }

    fn born() -> Self {
        Self {
            days_untill_repro: 8,
            status: FishStatus::NewBorn,
        }
    }

    fn pass_a_day(&mut self) {
        match self.status {
            FishStatus::Normal => {
                if self.days_untill_repro == 0 {
                    self.days_untill_repro = 6;
                    self.status = FishStatus::RecentlyGaveBirth;
                } else {
                    self.days_untill_repro -= 1;
                    self.status = FishStatus::Normal;
                }
            }
            FishStatus::NewBorn | FishStatus::RecentlyGaveBirth => {
                self.days_untill_repro -= 1;
                self.status = FishStatus::Normal;
            }
        }
    }

    fn status(&self) -> FishStatus {
        self.status
    }
}

struct School(Vec<LanternFish>);

impl School {
    fn pass_a_day(&mut self) {
        let mut births = 0;
        self.0.iter_mut().for_each(|f| {
            f.pass_a_day();
            if f.status() == FishStatus::RecentlyGaveBirth {
                births += 1;
            }
        });
        (0..births).into_iter().for_each(|_| {
            self.0.push(LanternFish::born());
        });
    }
}

fn fish_count_after_x_days(days: usize, input_text: &str) -> usize {
    let fishes: Vec<LanternFish> = input_text
        .split(',')
        .map(|c| LanternFish::from_until_days(c.parse().unwrap()))
        .collect();
    let mut school = School(fishes);
    (0..days).into_iter().for_each(|_| {
        school.pass_a_day();
    });
    school.0.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_18_days() {
        assert_eq!(26, fish_count_after_x_days(18, INPUT));
    }

    #[test]
    fn after_80_days() {
        assert_eq!(5934, fish_count_after_x_days(80, INPUT));
    }

    const INPUT: &str = r"3,4,3,1,2";
}
