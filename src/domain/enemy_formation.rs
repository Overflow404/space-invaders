use crate::domain::enemy::{Enemy, EnemyId};
use tracing::info;

pub const NUMBER_OF_STEPS_ON_X_AXE: usize = 41;
pub const COLUMNS: usize = 11;
pub const ROWS: usize = 5;
const FREE_MOVING_SPACE_ON_X_AXE: usize = NUMBER_OF_STEPS_ON_X_AXE - COLUMNS;

pub struct EnemyFormation {
    enemies: Vec<Vec<Option<Enemy>>>,
    position: (usize, usize),
    direction: MovingDirection,
    status: FormationStatus,
    enemies_alive: usize,
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
    Annihilated,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemyFormation {
    pub fn new() -> Self {
        let mut enemies: Vec<Vec<Option<Enemy>>> = vec![];
        let mut id = 1;

        for _ in 0..ROWS {
            let mut row = vec![];
            for _ in 0..COLUMNS {
                row.push(Some(Enemy::new(id)));
                id += 1;
            }
            enemies.push(row);
        }

        EnemyFormation {
            enemies,
            position: (0, 0),
            direction: MovingDirection::ToRight,
            status: FormationStatus::Assembled,
            enemies_alive: COLUMNS * ROWS,
        }
    }

    pub fn advance(&mut self) {
        if self.status == FormationStatus::Breached {
            info!("Enemy formation already breached");
            return;
        }

        let current_x = self.position.0;
        const BREACH_Y_LIMIT: usize = 14;

        let mut is_breaching = || {
            return if self.position.1 + 1 >= BREACH_Y_LIMIT {
                self.status = FormationStatus::Breached;
                info!("Enemy formation breached!");
                true
            } else {
                false
            };
        };

        match self.direction {
            MovingDirection::ToRight => {
                if current_x < FREE_MOVING_SPACE_ON_X_AXE {
                    self.position.0 += 1;
                } else {
                    if is_breaching() {
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
                    if is_breaching() {
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

    pub fn get_enemies(&self) -> &Vec<Vec<Option<Enemy>>> {
        &self.enemies
    }

    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    pub fn get_status(&self) -> FormationStatus {
        self.status
    }

    pub fn kill(&mut self, id: EnemyId) {
        let id_value = id.value();

        if id_value == 0 || id_value > COLUMNS * ROWS {
            return;
        }

        let id_index = id_value - 1;
        let row = id_index / COLUMNS;
        let col = id_index % COLUMNS;

        if row < ROWS
            && let Some(enemy) = &self.enemies[row][col]
            && enemy.get_id() == id
        {
            self.enemies[row][col] = None;
            self.enemies_alive -= 1;
        }

        if self.enemies_alive == 0 {
            self.status = FormationStatus::Annihilated;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_formation() -> EnemyFormation {
        EnemyFormation::new()
    }

    fn advance_formation_n_times(formation: &mut EnemyFormation, n: usize) {
        for _ in 0..n {
            formation.advance();
        }
    }

    fn advance_until_breached(formation: &mut EnemyFormation) {
        while formation.get_status() != FormationStatus::Breached {
            formation.advance();
        }
    }

    #[test]
    fn new_formation_starts_assembled_at_origin() {
        let formation = create_formation();

        assert_eq!(formation.get_position(), (0, 0));
        assert_eq!(formation.get_status(), FormationStatus::Assembled);

        let enemies = formation.get_enemies();
        assert_eq!(enemies.len(), 5);
        assert_eq!(enemies[0].len(), 11);
    }

    #[test]
    fn advancing_formation_moves_right_and_changes_status() {
        let mut formation = create_formation();

        formation.advance();

        assert_eq!(formation.get_position(), (1, 0));
        assert_eq!(formation.get_status(), FormationStatus::Advancing);
    }

    #[test]
    fn formation_drops_down_and_reverses_at_right_boundary() {
        let mut formation = create_formation();

        advance_formation_n_times(&mut formation, FREE_MOVING_SPACE_ON_X_AXE);

        assert_eq!(formation.get_position(), (30, 0));

        formation.advance();

        assert_eq!(formation.get_position(), (30, 1));
    }

    #[test]
    fn formation_moves_left_after_hitting_right_boundary() {
        let mut formation = create_formation();

        advance_formation_n_times(&mut formation, FREE_MOVING_SPACE_ON_X_AXE + 1);

        let (x_before, y_before) = formation.get_position();
        formation.advance();
        let (x_after, y_after) = formation.get_position();

        assert!(x_after < x_before);
        assert_eq!(y_after, y_before);
    }

    #[test]
    fn formation_drops_down_and_reverses_at_left_boundary() {
        let mut formation = create_formation();

        advance_formation_n_times(&mut formation, FREE_MOVING_SPACE_ON_X_AXE);
        formation.advance();
        advance_formation_n_times(&mut formation, FREE_MOVING_SPACE_ON_X_AXE);

        assert_eq!(formation.get_position(), (0, 1));

        formation.advance();

        assert_eq!(formation.get_position(), (0, 2));
    }

    #[test]
    fn formation_becomes_breached_when_reaching_bottom() {
        let mut formation = create_formation();

        advance_until_breached(&mut formation);

        assert_eq!(formation.get_position().1, 13);
        assert_eq!(formation.get_status(), FormationStatus::Breached);
    }

    #[test]
    fn breached_formation_stops_advancing() {
        let mut formation = create_formation();

        advance_until_breached(&mut formation);

        let position_at_breach = formation.get_position();

        assert_eq!(position_at_breach.1, 13);
        assert_eq!(formation.get_status(), FormationStatus::Breached);

        formation.advance();

        assert_eq!(formation.get_position(), position_at_breach);
        assert_eq!(formation.get_status(), FormationStatus::Breached);
    }

    #[test]
    fn killing_enemy_removes_it_from_formation() {
        let mut formation = create_formation();

        formation.kill(EnemyId::new(3));

        let enemies = formation.get_enemies();
        assert!(enemies[0][2].is_none());
    }

    #[test]
    fn killing_all_enemies_annihilates_formation() {
        let mut formation = create_formation();

        (1..=55).for_each(|id| formation.kill(EnemyId::new(id)));

        assert_eq!(formation.get_status(), FormationStatus::Annihilated);
    }

    #[test]
    fn killing_invalid_enemy_id_does_nothing() {
        let mut formation = create_formation();

        formation.kill(EnemyId::new(999));

        let enemies = formation.get_enemies();
        let alive_count = enemies
            .iter()
            .flat_map(|row| row.iter())
            .filter(|e| e.is_some())
            .count();
        assert_eq!(alive_count, 55);
    }

    #[test]
    fn killing_zero_id_does_nothing() {
        let mut formation = create_formation();

        formation.kill(EnemyId::new(0));

        let enemies = formation.get_enemies();
        let alive_count = enemies
            .iter()
            .flat_map(|row| row.iter())
            .filter(|e| e.is_some())
            .count();
        assert_eq!(alive_count, 55);
    }
}
