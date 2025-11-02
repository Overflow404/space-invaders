use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone, Copy)]
pub struct Enemy {
    id: usize,
}

impl Enemy {
    pub fn new(id: usize) -> Self {
        Enemy { id }
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
}
