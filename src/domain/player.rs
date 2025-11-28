pub struct Player {
    is_firing: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player { is_firing: false }
    }

    pub fn toggle_fire(&mut self) {
        self.is_firing = !self.is_firing
    }

    pub fn is_firing(&self) -> bool {
        self.is_firing
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::player::Player;

    #[test]
    fn should_create_player() {
        let player = Player::new();
        assert!(!player.is_firing, "Initial firing state should be false");
    }

    #[test]
    fn toggle_fire_switches_state() {
        let mut player = Player::new();

        assert!(!player.is_firing(), "Initial firing state should be false");

        player.toggle_fire();
        assert!(player.is_firing(), "Player should start firing");

        player.toggle_fire();
        assert!(!player.is_firing(), "Player should stop firing");
    }
}
