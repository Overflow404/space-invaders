#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnemyId(usize);

impl EnemyId {
    pub fn new(id: usize) -> Self {
        EnemyId(id)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    id: EnemyId,
    is_firing: bool,
}

impl Enemy {
    pub fn new(id: usize) -> Self {
        Enemy {
            id: EnemyId::new(id),
            is_firing: false,
        }
    }

    pub fn get_id(&self) -> EnemyId {
        self.id
    }

    pub fn is_firing(&self) -> bool {
        self.is_firing
    }

    pub fn toggle_fire(&mut self) {
        self.is_firing = !self.is_firing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_enemy_with_id(id: usize) -> Enemy {
        Enemy::new(id)
    }

    #[test]
    fn new_enemy_has_assigned_id() {
        let enemy = create_enemy_with_id(42);
        assert_eq!(enemy.get_id(), EnemyId::new(42));
    }

    #[test]
    fn new_enemy_is_not_firing() {
        let enemy = create_enemy_with_id(1);
        assert!(!enemy.is_firing());
    }

    #[test]
    fn toggling_fire_starts_firing() {
        let mut enemy = create_enemy_with_id(1);
        enemy.toggle_fire();
        assert!(enemy.is_firing());
    }

    #[test]
    fn toggling_fire_twice_stops_firing() {
        let mut enemy = create_enemy_with_id(1);
        enemy.toggle_fire();
        enemy.toggle_fire();
        assert!(!enemy.is_firing());
    }
}
