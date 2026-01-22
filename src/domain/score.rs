pub struct Score {
    current: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}

impl Score {
    pub fn new() -> Self {
        Score { current: 0 }
    }

    pub fn increment(&mut self, points: u32) {
        self.current = self.current.saturating_add(points);
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_score() -> Score {
        Score::new()
    }

    fn create_score_with_value(value: u32) -> Score {
        let mut score = Score::new();
        score.increment(value);
        score
    }

    #[test]
    fn new_score_starts_at_zero() {
        let score = create_score();
        assert_eq!(score.get_current(), 0);
    }

    #[test]
    fn incrementing_score_increases_current_value() {
        let mut score = create_score();
        score.increment(10);
        assert_eq!(score.get_current(), 10);
    }

    #[test]
    fn incrementing_score_multiple_times_accumulates_points() {
        let mut score = create_score();
        score.increment(10);
        score.increment(25);
        assert_eq!(score.get_current(), 35);
    }

    #[test]
    fn resetting_score_returns_to_zero() {
        let mut score = create_score_with_value(100);
        score.reset();
        assert_eq!(score.get_current(), 0);
    }

    #[test]
    fn score_saturates_at_maximum_value() {
        let mut score = create_score_with_value(u32::MAX - 10);
        score.increment(20);
        assert_eq!(score.get_current(), u32::MAX);
    }

    #[test]
    fn score_handles_large_increments_without_overflow() {
        let mut score = create_score();
        score.increment(u32::MAX);
        assert_eq!(score.get_current(), u32::MAX);
    }
}
