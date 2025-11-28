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
        self.current += points;
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
    use crate::domain::score::Score;

    #[test]
    fn should_create_score() {
        let score = Score::new();
        assert_eq!(score.current, 0);
    }

    #[test]
    fn should_increment_score() {
        let mut score = Score::new();
        score.increment(10);
        assert_eq!(score.current, 10);
    }

    #[test]
    fn should_reset_score() {
        let mut score = Score::new();
        score.increment(10);
        score.reset();
        assert_eq!(score.current, 0);
    }

    #[test]
    fn should_get_current_score() {
        let mut score = Score::new();
        score.increment(10);
        assert_eq!(score.get_current(), 10);
    }
}
