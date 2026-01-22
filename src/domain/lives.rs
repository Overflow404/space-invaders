pub const DEFAULT_LIVES: u8 = 3;

pub struct Lives {
    current: u8,
}

impl Default for Lives {
    fn default() -> Self {
        Self::new()
    }
}

impl Lives {
    pub fn new() -> Self {
        Lives {
            current: DEFAULT_LIVES,
        }
    }

    pub fn decrement(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn get_current(&self) -> u8 {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = DEFAULT_LIVES;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_lives() -> Lives {
        Lives::new()
    }

    fn decrement_n_times(lives: &mut Lives, n: u8) {
        for _ in 0..n {
            lives.decrement();
        }
    }

    #[test]
    fn new_lives_starts_at_three() {
        let lives = create_lives();
        assert_eq!(lives.get_current(), 3);
    }

    #[test]
    fn decrementing_lives_decreases_current_value() {
        let mut lives = create_lives();
        lives.decrement();
        assert_eq!(lives.get_current(), 2);
    }

    #[test]
    fn resetting_lives_returns_to_three() {
        let mut lives = create_lives();
        lives.decrement();
        lives.reset();
        assert_eq!(lives.get_current(), 3);
    }

    #[test]
    fn exhausted_lives_cannot_be_decremented_below_zero() {
        let mut lives = create_lives();
        decrement_n_times(&mut lives, 4);

        assert_eq!(lives.get_current(), 0);
    }
}
