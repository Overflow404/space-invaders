
pub struct Lives {
    current: u32,
}

impl Default for Lives {
    fn default() -> Self {
        Self::new()
    }
}

impl Lives {
    pub fn new() -> Self {
        Lives { current: 3 }
    }

    pub fn decrement(&mut self) {
        self.current -= 1;
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = 3;
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::lives::Lives;

    #[test]
    fn should_create_new_lives() {
        let lives = Lives::new();
        assert_eq!(lives.get_current(), 3);
    }

    #[test]
    fn should_decrement_lives() {
        let mut lives = Lives::new();
        lives.decrement();
        assert_eq!(lives.get_current(), 2);
    }

    #[test]
    fn should_reset_lives() {
        let mut lives = Lives::new();
        lives.decrement();
        lives.reset();
        assert_eq!(lives.get_current(), 3);
    }
}
