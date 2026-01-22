use crate::domain::weapons::{Fireable, WeaponState};

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
    weapon_state: WeaponState,
}

impl Enemy {
    pub fn new(id: usize) -> Self {
        Enemy {
            id: EnemyId::new(id),
            weapon_state: WeaponState::Ready,
        }
    }

    pub fn get_id(&self) -> EnemyId {
        self.id
    }
}

impl Fireable for Enemy {
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

    #[test]
    fn new_enemy_can_fire() {
        let enemy = create_enemy_with_id(1);
        assert!(enemy.can_fire());
    }

    #[test]
    fn enemy_cannot_fire_while_firing() {
        let mut enemy = create_enemy_with_id(1);
        enemy.start_firing();
        assert!(!enemy.can_fire());
    }

    #[test]
    fn start_firing_changes_state_to_firing() {
        let mut enemy = create_enemy_with_id(1);
        enemy.start_firing();
        assert!(enemy.is_firing());
    }

    #[test]
    fn reload_changes_state_to_ready() {
        let mut enemy = create_enemy_with_id(1);
        enemy.start_firing();
        enemy.reload();
        assert!(!enemy.is_firing());
        assert!(enemy.can_fire());
    }
}
