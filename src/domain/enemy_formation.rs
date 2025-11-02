use std::fmt;

use tracing::debug;

use crate::domain::enemy::Enemy;

const ROWS: usize = 10;
const COLUMNS: usize = 41;

pub struct EnemyFormation {
    enemies: Vec<Vec<Option<Enemy>>>,
    moving_direction: MovingDirection,
    state: EnemyFormationState,
}

#[derive(PartialEq)]
enum MovingDirection {
    ToLeft,
    ToRight,
}

#[derive(PartialEq, Debug)]
enum EnemyFormationState {
    StartPosition,
    Running,
    EnemiesWon,
}

impl fmt::Debug for EnemyFormation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "     j: ")?;
        for j in 0..COLUMNS {
            write!(f, "{:^3}", j)?;
        }
        writeln!(f)?;

        write!(f, "       ")?;
        for _ in 0..COLUMNS {
            write!(f, "---")?;
        }
        writeln!(f)?;

        for (i, row) in self.enemies.iter().enumerate() {
            write!(f, " i: {:<2} |", i)?;

            for cell in row.iter() {
                let symbol = match cell {
                    Some(_) => "E",
                    None => ".",
                };
                write!(f, " {:1} ", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl EnemyFormation {
    pub fn new() -> Self {
        let mut enemies = vec![vec![None; COLUMNS]; ROWS];

        let mut id = 1;

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                if j >= 15 && j <= 26 && i < 5 {
                    enemies[i][j] = Some(Enemy::new(id));
                    id += 1;
                }
            }
        }

        EnemyFormation {
            enemies,
            moving_direction: MovingDirection::ToRight,
            state: EnemyFormationState::StartPosition,
        }
    }

    pub fn advance_enemies(&mut self) {
        if self.state == EnemyFormationState::EnemiesWon {
            debug!("Enemy formation already reached end, not advancing!");
            debug!("{:?}", self);
            return;
        }

        match self.moving_direction {
            MovingDirection::ToRight => self.move_all_enemies_to_the_right(),
            MovingDirection::ToLeft => self.move_all_enemies_to_the_left(),
        }

        if self.enemies_won() {
            self.state = EnemyFormationState::EnemiesWon;

            debug!("Enemy formation just reached end, they won!");
            debug!("{:?}", self);
            return;
        }

        self.state = EnemyFormationState::Running
    }

    fn enemies_won(&self) -> bool {
        let bottom_leftmost: Option<(usize, usize)> = self
            .enemies
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, enemy)| enemy.as_ref().map(|_| (i, j)))
            })
            .max_by_key(|(i, j)| (*i, std::cmp::Reverse(*j)));

        if let Some((i, j)) = bottom_leftmost {
            if i == ROWS - 1 && j == 0 {
                return true;
            }
        }

        false
    }

    fn move_all_enemies_to_the_left(&mut self) {
        let top_leftmost: Option<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(_, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, enemy)| enemy.as_ref().map(|_| j))
            })
            .min_by_key(|j| *j);

        if let Some(j) = top_leftmost {
            if j == 0 {
                self.moving_direction = MovingDirection::ToRight;
                let mut new_enemies = vec![vec![None; COLUMNS]; ROWS];
                for i in 0..ROWS - 1 {
                    for j in 0..COLUMNS {
                        new_enemies[i + 1][j] = self.enemies[i][j].clone();
                    }
                }
                self.enemies = new_enemies;
            } else {
                let mut new_enemies = vec![vec![None; COLUMNS]; ROWS];
                for i in 0..ROWS {
                    for j in 1..COLUMNS {
                        new_enemies[i][j - 1] = self.enemies[i][j].clone();
                    }
                }
                self.enemies = new_enemies;
            }
        }
    }

    fn move_all_enemies_to_the_right(&mut self) {
        let top_rightmost: Option<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(_, row)| {
                row.iter()
                    .enumerate()
                    .rev()
                    .find_map(|(j, enemy)| enemy.as_ref().map(|_| j))
            })
            .max_by_key(|j| *j);

        if let Some(j) = top_rightmost {
            if j == COLUMNS - 1 {
                self.moving_direction = MovingDirection::ToLeft;
                let mut new_enemies = vec![vec![None; COLUMNS]; ROWS];
                for i in 0..ROWS - 1 {
                    for j in 0..COLUMNS {
                        new_enemies[i + 1][j] = self.enemies[i][j].clone();
                    }
                }
                self.enemies = new_enemies;
            } else {
                let mut new_enemies = vec![vec![None; COLUMNS]; ROWS];
                for i in 0..ROWS {
                    for j in 0..COLUMNS - 1 {
                        new_enemies[i][j + 1] = self.enemies[i][j].clone();
                    }
                }
                self.enemies = new_enemies;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_starting_enemy_formation() {
        let formation = EnemyFormation::new();

        assert_eq!(formation.enemies.len(), ROWS);
        assert_eq!(formation.enemies[0].len(), COLUMNS);
        assert_eq!(formation.state, EnemyFormationState::StartPosition);

        for i in 0..(ROWS - 1) {
            for j in 0..(COLUMNS - 1) {
                if i >= 5 {
                    assert!(formation.enemies[i][j].is_none());
                } else {
                    if j >= 15 && j <= 26 {
                        assert!(formation.enemies[i][j].is_some());
                    } else {
                        assert!(formation.enemies[i][j].is_none());
                    }
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_right_when_there_is_enough_space() {
        let mut formation = EnemyFormation::new();
        formation.advance_enemies();

        assert_eq!(formation.state, EnemyFormationState::Running);

        for i in 0..(ROWS - 1) {
            for j in 0..(COLUMNS - 1) {
                if i >= 5 {
                    assert!(formation.enemies[i][j].is_none());
                } else {
                    if j >= 16 && j <= 27 {
                        assert!(formation.enemies[i][j].is_some());
                    } else {
                        assert!(formation.enemies[i][j].is_none());
                    }
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_next_row_when_there_is_no_right_space() {
        let mut formation = EnemyFormation::new();

        for _ in 0..15 {
            formation.advance_enemies();
        }

        assert_eq!(formation.state, EnemyFormationState::Running);

        for i in 0..(ROWS - 1) {
            for j in 0..(COLUMNS - 1) {
                if i == 0 || i >= 6 {
                    assert!(formation.enemies[i][j].is_none());
                } else {
                    if j >= 29 && j <= 40 {
                        assert!(formation.enemies[i][j].is_some());
                    } else {
                        assert!(formation.enemies[i][j].is_none());
                    }
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_left_when_there_is_enough_space() {
        let mut formation = EnemyFormation::new();

        for _ in 0..16 {
            formation.advance_enemies();
        }

        assert_eq!(formation.state, EnemyFormationState::Running);

        for i in 0..(ROWS - 1) {
            for j in 0..(COLUMNS - 1) {
                if i == 0 || i >= 6 {
                    assert!(formation.enemies[i][j].is_none());
                } else {
                    if j >= 28 && j <= 39 {
                        assert!(formation.enemies[i][j].is_some());
                    } else {
                        assert!(formation.enemies[i][j].is_none());
                    }
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_next_row_when_there_is_no_left_space() {
        let mut formation = EnemyFormation::new();

        for _ in 0..45 {
            formation.advance_enemies();
        }

        assert_eq!(formation.state, EnemyFormationState::Running);

        for i in 0..(ROWS - 1) {
            for j in 0..(COLUMNS - 1) {
                if i <= 1 || i >= 7 {
                    assert!(formation.enemies[i][j].is_none());
                } else {
                    if j <= 11 {
                        assert!(formation.enemies[i][j].is_some());
                    } else {
                        assert!(formation.enemies[i][j].is_none());
                    }
                }
            }
        }
    }

    #[test]
    fn should_set_end_state_when_enemies_reach_the_bottom_right() {
        let mut formation = EnemyFormation::new();

        for _ in 0..164 {
            formation.advance_enemies();
        }

        assert_eq!(formation.state, EnemyFormationState::EnemiesWon);
    }

    #[test]
    fn should_not_advance_anymore_when_end_state_is_reached() {
        let mut formation = EnemyFormation::new();

        for _ in 0..165 {
            formation.advance_enemies();
        }

        assert_eq!(formation.state, EnemyFormationState::EnemiesWon);
    }
}
