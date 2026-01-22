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
    use super::*;

    fn create_player() -> Player {
        Player::new()
    }

    #[test]
    fn new_player_is_not_firing() {
        let player = create_player();
        assert!(!player.is_firing());
    }

    #[test]
    fn toggling_fire_starts_firing() {
        let mut player = create_player();
        player.toggle_fire();
        assert!(player.is_firing());
    }

    #[test]
    fn toggling_fire_twice_stops_firing() {
        let mut player = create_player();
        player.toggle_fire();
        player.toggle_fire();
        assert!(!player.is_firing());
    }
}
