pub struct SuperMemo2 {
    min_ease_factor: f64,
    max_grade: u64,
    default_interval: u64,
}

impl SuperMemo2 {
    fn new() -> Self {
        Self {
            max_grade: 5,
            min_ease_factor: 1.3,
            default_interval: 1,
        }
    }
}

pub trait Calculate {
    fn calc(
        &self,
        grade: u64,
        repetitions: u64,
        ease_factor: f64,
        interval: u64,
    ) -> (u64, u64, f64, u64);
}

pub trait Create {
    fn new_memo(&self, memo: SuperMemo2) -> Self;
}

impl Create for SuperMemo2 {
    fn new_memo(&self, memo: SuperMemo2) -> Self {
        memo
    }
}

impl SuperMemo2 {
    pub fn handle(&self) -> () {}
}

impl Calculate for SuperMemo2 {
    fn calc(
        &self,
        grade: u64,
        mut repetitions: u64,
        mut ease_factor: f64,
        mut interval: u64,
    ) -> (u64, u64, f64, u64) {
        assert!(self.max_grade >= grade, "given grade exceeds maximum");
        assert!(
            self.min_ease_factor <= ease_factor,
            "given ease_factor below min"
        );
        let grade_f = grade as f64;
        if grade > 3 {
            if repetitions == 0 {
                interval = self.default_interval;
            } else if repetitions == 1 {
                interval = 6;
            } else {
                let interval_f = interval as f64;
                interval = (interval_f * ease_factor).round() as u64;
            }
            repetitions += 1;
            ease_factor += 0.1 - (5f64 - grade_f) * (0.08 + (5f64 - grade_f) * 0.02);
            if ease_factor < self.min_ease_factor {
                ease_factor = self.min_ease_factor;
            }
        } else {
            repetitions = 0;
            interval = 1;
        }
        return (grade, repetitions, ease_factor, interval);
    }
}

pub fn super_memo2(
    grade: u64,
    repetitions: u64,
    ease_factor: f64,
    interval: u64,
) -> (u64, u64, f64, u64) {
    SuperMemo2::new().calc(grade, repetitions, ease_factor, interval)
}

#[cfg(test)]
mod tests {
    use super::{super_memo2, Calculate, SuperMemo2};

    #[test]
    fn incorrect_response_first_try() {
        let res = SuperMemo2::new().calc(0, 0, 2.5, 0);
        assert_eq!(res, (0, 0, 2.5, 1));
    }
    #[test]
    fn correct_response_second_time() {
        assert_eq!(super_memo2(5, 2, 1.3, 6), (5, 3, 1.4000000000000001, 8))
    }

    #[should_panic]
    #[test]
    fn grade_over_max_length() {
        super_memo2(50, 2, 1.3, 6);
    }
}
