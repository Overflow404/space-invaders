use crate::domain::enemy::Enemy;
use tracing::info;

pub const NUMBER_OF_STEPS_ON_X_AXE: usize = 41;
pub const COLUMNS: usize = 11;
pub const ROWS: usize = 5;
const FREE_MOVING_SPACE_ON_X_AXE: usize = NUMBER_OF_STEPS_ON_X_AXE - COLUMNS;

pub struct EnemyFormation {
    enemies: Vec<Vec<Enemy>>,
    position: (usize, usize),
    direction: MovingDirection,
    status: FormationStatus,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovingDirection {
    ToLeft,
    ToRight,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FormationStatus {
    Assembled,
    Advancing,
    Breached,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemyFormation {
    pub fn new() -> Self {
        let mut enemies: Vec<Vec<Enemy>> = vec![];
        let mut id = 1;

        for _ in 0..ROWS {
            let mut row = vec![];
            for _ in 0..COLUMNS {
                row.push(Enemy::new(id));
                id += 1;
            }
            enemies.push(row);
        }

        EnemyFormation {
            enemies,
            position: (0, 0),
            direction: MovingDirection::ToRight,
            status: FormationStatus::Assembled,
        }
    }

    pub fn advance_enemies(&mut self) {
        if self.status == FormationStatus::Breached {
            info!("Enemy formation already breached");
            return;
        }

        let current_x = self.position.0;
        const BREACH_Y_LIMIT: usize = 14;

        match self.direction {
            MovingDirection::ToRight => {
                if current_x < FREE_MOVING_SPACE_ON_X_AXE {
                    self.position.0 += 1;
                } else {
                    if self.position.1 + 1 >= BREACH_Y_LIMIT {
                        self.status = FormationStatus::Breached;
                        info!("Enemy formation breached");
                        return;
                    }

                    self.position.1 += 1;
                    self.direction = MovingDirection::ToLeft;
                }
            }
            MovingDirection::ToLeft => {
                if current_x > 0 {
                    self.position.0 -= 1;
                } else {
                    if self.position.1 + 1 >= BREACH_Y_LIMIT {
                        self.status = FormationStatus::Breached;
                        info!("Enemy formation breached");
                        return;
                    }

                    self.position.1 += 1;
                    self.direction = MovingDirection::ToRight;
                }
            }
        }

        self.status = FormationStatus::Advancing;

        info!(
            "Formation moved to {:?}, direction: {:?}",
            self.position, self.direction
        );
    }

    pub fn get_enemies(&self) -> &Vec<Vec<Enemy>> {
        &self.enemies
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }
}
#[cfg(test)]
mod tests {
    use crate::domain::enemy_formation::{
        EnemyFormation, FormationStatus, MovingDirection, FREE_MOVING_SPACE_ON_X_AXE,
    };

    #[test]
    fn should_create_formation() {
        let formation = EnemyFormation::new();

        assert_eq!(formation.enemies.len(), 5);
        assert_eq!(formation.enemies[0].len(), 11);

        assert_eq!(formation.position, (0, 0));
        assert_eq!(formation.direction, MovingDirection::ToRight);
        assert_eq!(formation.status, FormationStatus::Assembled);
    }

    #[test]
    fn should_advance_enemies_to_the_right_when_there_is_space() {
        let mut formation = EnemyFormation::new();

        formation.advance_enemies();

        assert_eq!(formation.position, (1, 0));
        assert_eq!(formation.direction, MovingDirection::ToRight);
        assert_eq!(formation.status, FormationStatus::Advancing);
    }

    #[test]
    fn should_hit_right_wall_and_drop_down() {
        let mut formation = EnemyFormation::new();

        for _ in 0..FREE_MOVING_SPACE_ON_X_AXE {
            formation.advance_enemies();
        }

        assert_eq!(formation.position, (30, 0));
        assert_eq!(formation.direction, MovingDirection::ToRight);

        formation.advance_enemies();

        assert_eq!(formation.position, (30, 1));
        assert_eq!(formation.direction, MovingDirection::ToLeft);
    }

    #[test]
    fn should_advance_enemies_to_the_left_when_there_is_space() {
        let mut formation = EnemyFormation::new();

        for _ in 0..(FREE_MOVING_SPACE_ON_X_AXE + 1) {
            formation.advance_enemies();
        }

        formation.advance_enemies();

        assert_eq!(formation.direction, MovingDirection::ToLeft);
    }

    #[test]
    fn should_hit_left_wall_and_drop_down() {
        let mut formation = EnemyFormation::new();

        for _ in 0..FREE_MOVING_SPACE_ON_X_AXE {
            formation.advance_enemies();
        }
        formation.advance_enemies();
        for _ in 0..FREE_MOVING_SPACE_ON_X_AXE {
            formation.advance_enemies();
        }

        assert_eq!(formation.position, (0, 1));
        assert_eq!(formation.direction, MovingDirection::ToLeft);

        formation.advance_enemies();

        assert_eq!(formation.position, (0, 2));
        assert_eq!(formation.direction, MovingDirection::ToRight);
    }

    #[test]
    fn should_detect_breach_when_reaching_bottom() {
        let mut formation = EnemyFormation::new();

        while formation.status != FormationStatus::Breached {
            formation.advance_enemies();
        }

        assert_eq!(formation.position.1, 13);
        assert_eq!(formation.status, FormationStatus::Breached);
    }

    #[test]
    fn should_not_advance_anymore_when_breached() {
        let mut formation = EnemyFormation::new();

        while formation.status != FormationStatus::Breached {
            formation.advance_enemies();
        }

        let position_at_breach = formation.position;

        assert_eq!(position_at_breach.1, 13);
        assert_eq!(formation.status, FormationStatus::Breached);

        formation.advance_enemies();

        assert_eq!(formation.position, position_at_breach);
        assert_eq!(formation.status, FormationStatus::Breached);
    }
}
