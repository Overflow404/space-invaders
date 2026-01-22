pub const DEFAULT_SHIELD_HEALTH: u8 = 100;
pub const MIN_SHIELD_HEALTH: u8 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Shield {
    health: u8,
}

impl Default for Shield {
    fn default() -> Self {
        Self::new()
    }
}

impl Shield {
    pub fn new() -> Self {
        Shield {
            health: DEFAULT_SHIELD_HEALTH,
        }
    }

    pub fn damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn is_destroyed(&self) -> bool {
        self.health == MIN_SHIELD_HEALTH
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_shield() -> Shield {
        Shield::new()
    }

    fn create_damaged_shield(damage: u8) -> Shield {
        let mut shield = Shield::new();
        shield.damage(damage);
        shield
    }

    #[test]
    fn new_shield_starts_at_full_health() {
        let shield = create_shield();
        assert_eq!(shield.get_health(), DEFAULT_SHIELD_HEALTH);
    }

    #[test]
    fn new_shield_is_not_destroyed() {
        let shield = create_shield();
        assert!(!shield.is_destroyed());
    }

    #[test]
    fn damaging_shield_reduces_health() {
        let mut shield = create_shield();
        shield.damage(25);
        assert_eq!(shield.get_health(), 75);
    }

    #[test]
    fn damaging_shield_multiple_times_accumulates_damage() {
        let mut shield = create_shield();
        shield.damage(30);
        shield.damage(20);
        assert_eq!(shield.get_health(), 50);
    }

    #[test]
    fn shield_destroyed_when_health_reaches_zero() {
        let mut shield = create_shield();
        shield.damage(DEFAULT_SHIELD_HEALTH);
        assert!(shield.is_destroyed());
        assert_eq!(shield.get_health(), MIN_SHIELD_HEALTH);
    }

    #[test]
    fn shield_health_cannot_go_below_zero() {
        let mut shield = create_shield();
        shield.damage(DEFAULT_SHIELD_HEALTH + 50);
        assert_eq!(shield.get_health(), MIN_SHIELD_HEALTH);
        assert!(shield.is_destroyed());
    }

    #[test]
    fn partially_damaged_shield_not_destroyed() {
        let shield = create_damaged_shield(50);
        assert!(!shield.is_destroyed());
        assert_eq!(shield.get_health(), 50);
    }
}
