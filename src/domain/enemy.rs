#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    id: usize,
    is_firing: bool,
}

impl Enemy {
    pub fn is_firing(&self) -> bool {
        self.is_firing
    }

    pub fn toggle_fire(&mut self) {
        self.is_firing = !self.is_firing;
    }
}

impl Enemy {
    pub fn new(id: usize) -> Self {
        Enemy { id, is_firing: false }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::enemy::Enemy;

    #[test]
    fn should_create_enemy() {
        let enemy = Enemy::new(0);
        assert_eq!(enemy.id, 0);
    }

    #[test]
    fn should_get_enemy_id() {
        let enemy = Enemy::new(25);
        assert_eq!(enemy.get_id(), 25);
    }

    #[test]
    fn toggle_fire_switches_state() {
        let mut enemy = Enemy::new(1);

        assert!(!enemy.is_firing(), "Initial state should be false");

        enemy.toggle_fire();
        assert!(enemy.is_firing(), "Player should fire after first toggle");

        enemy.toggle_fire();
        assert!(!enemy.is_firing(), "Player should stop firing after second toggle");
    }
}
