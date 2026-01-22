use crate::domain::weapons::{Fireable, WeaponState};

pub struct Player {
    weapon_state: WeaponState,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            weapon_state: WeaponState::Ready,
        }
    }
}

impl Fireable for Player {
    fn start_firing(&mut self) {
        self.weapon_state = WeaponState::Firing;
    }

    fn reload(&mut self) {
        self.weapon_state = WeaponState::Ready;
    }

    fn can_fire(&self) -> bool {
        self.weapon_state == WeaponState::Ready
    }

    fn is_firing(&self) -> bool {
        self.weapon_state == WeaponState::Firing
    }

    fn toggle_fire(&mut self) {
        self.weapon_state = match self.weapon_state {
            WeaponState::Ready => WeaponState::Firing,
            WeaponState::Firing => WeaponState::Ready,
        };
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

    #[test]
    fn new_player_can_fire() {
        let player = create_player();
        assert!(player.can_fire());
    }

    #[test]
    fn player_cannot_fire_while_firing() {
        let mut player = create_player();
        player.start_firing();
        assert!(!player.can_fire());
    }

    #[test]
    fn start_firing_changes_state_to_firing() {
        let mut player = create_player();
        player.start_firing();
        assert!(player.is_firing());
    }

    #[test]
    fn reload_changes_state_to_ready() {
        let mut player = create_player();
        player.start_firing();
        player.reload();
        assert!(!player.is_firing());
        assert!(player.can_fire());
    }

    #[test]
    fn reloading_when_ready_keeps_state_ready() {
        let mut player = create_player();
        player.reload();
        assert!(player.can_fire());
    }
}
