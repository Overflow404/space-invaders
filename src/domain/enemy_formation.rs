use crate::domain::enemy::Enemy;
use std::fmt;
use tracing::debug;

const ROWS: usize = 15;
const COLUMNS: usize = 41;

const FIRST_ENEMY_Y: usize = 15;

const LAST_ENEMY_X: usize = 4;
const LAST_ENEMY_Y: usize = 25;

const ENEMIES_PER_ROW: usize = LAST_ENEMY_Y - FIRST_ENEMY_Y + 1;

pub struct EnemyFormation {
    enemies: Vec<Vec<Option<Enemy>>>,
    direction: MovingDirection,
    status: FormationStatus,
}

#[derive(PartialEq, Debug)]
enum MovingDirection {
    ToLeft,
    ToRight,
}

#[derive(PartialEq, Debug)]
enum FormationStatus {
    Assembled,
    Advancing,
    Breached,
}

impl fmt::Debug for EnemyFormation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n     y: ")?;
        for y in 0..COLUMNS {
            write!(f, "{:^3}", y)?;
        }
        writeln!(f)?;

        write!(f, "       ")?;
        for _ in 0..COLUMNS {
            write!(f, "---")?;
        }
        writeln!(f)?;

        for (i, row) in self.enemies.iter().enumerate() {
            write!(f, " x: {:<2} |", i)?;

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
impl Default for EnemyFormation {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemyFormation {
    pub fn new() -> Self {
        let mut enemies = vec![vec![None; COLUMNS]; ROWS];

        let mut id = 1;

        for (x, row) in enemies.iter_mut().enumerate() {
            for (y, enemy) in row.iter_mut().enumerate() {
                if (FIRST_ENEMY_Y..=LAST_ENEMY_Y).contains(&y) && x <= LAST_ENEMY_X {
                    *enemy = Some(Enemy::new(id));
                    id += 1;
                }
            }
        }

        let formation = EnemyFormation {
            enemies,
            direction: MovingDirection::ToRight,
            status: FormationStatus::Assembled,
        };

        debug!("Enemy formation created!");
        debug!("Number of enemies per row {}", ENEMIES_PER_ROW);
        debug!("Number of enemies {} enemies", id - 1);
        debug!("Starting direction: {:?}", formation.direction);
        debug!("Starting status: {:?}", formation.status);
        debug!("\n{:?}\n", formation);

        formation
    }

    pub fn advance_enemies(&mut self) {
        if self.status == FormationStatus::Breached {
            debug!("Enemy formation already reached end, not advancing!");
            debug!("{:?}", self);
            return;
        }

        match self.direction {
            MovingDirection::ToRight => self.move_all_enemies_to_the_right(),
            MovingDirection::ToLeft => self.move_all_enemies_to_the_left(),
        }

        if self.enemies_won() {
            self.status = FormationStatus::Breached;

            debug!("Enemy formation just reached end, they won!");
            debug!("{:?}", self);
            return;
        }

        self.status = FormationStatus::Advancing;

        debug!("Enemy formation advanced!");
        debug!("\n{:?}", self);
    }

    pub fn get_enemies(&self) -> &Vec<Vec<Option<Enemy>>> {
        &self.enemies
    }

    fn enemies_won(&self) -> bool {
        let bottom_rightmost: Option<(usize, usize)> = self
            .enemies
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, enemy)| enemy.as_ref().map(|_| (x, y)))
            })
            .max_by_key(|(x, y)| (*x, *y));

        if let Some((x, y)) = bottom_rightmost
            && x == ROWS - 1
            && y == COLUMNS - 1
        {
            return true;
        }

        false
    }

    fn move_all_enemies_to_the_left(&mut self) {
        let top_leftmost: Option<usize> = self
            .enemies
            .iter()
            .filter_map(|row| {
                row.iter()
                    .enumerate()
                    .find_map(|(y, enemy)| enemy.as_ref().map(|_| y))
            })
            .min_by_key(|y| *y);

        if let Some(y) = top_leftmost {
            if y == 0 {
                self.direction = MovingDirection::ToRight;

                let empty_row = vec![None; COLUMNS];

                let mut shifted_rows: Vec<Vec<Option<Enemy>>> =
                    self.enemies.iter().take(ROWS - 1).cloned().collect();

                shifted_rows.insert(0, empty_row);

                self.enemies = shifted_rows;
            } else {
                self.enemies = self
                    .enemies
                    .iter()
                    .map(|row| {
                        let mut new_row = row.clone();
                        new_row.remove(0);
                        new_row.push(None);
                        new_row
                    })
                    .collect();
            }
        }
    }

    fn move_all_enemies_to_the_right(&mut self) {
        let top_rightmost: Option<usize> = self
            .enemies
            .iter()
            .filter_map(|row| {
                row.iter()
                    .enumerate()
                    .rev()
                    .find_map(|(y, enemy)| enemy.as_ref().map(|_| y))
            })
            .max_by_key(|y| *y);

        if let Some(y) = top_rightmost {
            if y == COLUMNS - 1 {
                self.direction = MovingDirection::ToLeft;
                self.enemies.pop();
                self.enemies.insert(0, vec![None; COLUMNS]);
            } else {
                for row in self.enemies.iter_mut() {
                    row.pop();
                    row.insert(0, None);
                }
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
        assert_eq!(formation.status, FormationStatus::Assembled);
        assert_eq!(formation.direction, MovingDirection::ToRight);

        for x in 0..(ROWS - 1) {
            for y in 0..(COLUMNS - 1) {
                if x > LAST_ENEMY_X {
                    assert!(formation.enemies[x][y].is_none());
                } else if (FIRST_ENEMY_Y..=LAST_ENEMY_Y).contains(&y) {
                    assert!(formation.enemies[x][y].is_some());
                } else {
                    assert!(formation.enemies[x][y].is_none());
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_right_when_there_is_enough_space() {
        let mut formation = EnemyFormation::new();

        formation.advance_enemies();

        assert_eq!(formation.status, FormationStatus::Advancing);
        assert_eq!(formation.direction, MovingDirection::ToRight);

        for x in 0..(ROWS - 1) {
            for y in 0..(COLUMNS - 1) {
                if x > LAST_ENEMY_X {
                    assert!(formation.enemies[x][y].is_none());
                } else if y > FIRST_ENEMY_Y && y <= LAST_ENEMY_Y + 1 {
                    assert!(formation.enemies[x][y].is_some());
                } else {
                    assert!(formation.enemies[x][y].is_none());
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_next_row_when_there_is_no_right_space() {
        let mut formation = EnemyFormation::new();

        let row_jumps = 1;
        let fully_traversed_rows = 0;
        let steps_per_row = 30;

        let steps_to_reach_the_beginning_of_the_second_line =
            (fully_traversed_rows * steps_per_row) + row_jumps + FIRST_ENEMY_Y;

        for _ in 0..steps_to_reach_the_beginning_of_the_second_line {
            formation.advance_enemies();
        }

        assert_eq!(formation.status, FormationStatus::Advancing);
        assert_eq!(formation.direction, MovingDirection::ToLeft);

        for x in 0..(ROWS - 1) {
            for y in 0..(COLUMNS - 1) {
                if x == 0 || x >= 6 {
                    assert!(formation.enemies[x][y].is_none());
                } else if ((COLUMNS - ENEMIES_PER_ROW)..COLUMNS).contains(&y) {
                    assert!(formation.enemies[x][y].is_some());
                } else {
                    assert!(formation.enemies[x][y].is_none());
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_left_when_there_is_enough_space() {
        let mut formation = EnemyFormation::new();

        let row_jumps = 1;
        let fully_traversed_rows = 0;
        let steps_per_row = 30;

        let steps_to_reach_the_beginning_of_the_second_line =
            (fully_traversed_rows * steps_per_row) + row_jumps + FIRST_ENEMY_Y;

        for _ in 0..steps_to_reach_the_beginning_of_the_second_line {
            formation.advance_enemies();
        }

        formation.advance_enemies();

        assert_eq!(formation.status, FormationStatus::Advancing);
        assert_eq!(formation.direction, MovingDirection::ToLeft);

        for x in 0..(ROWS - 1) {
            for y in 0..(COLUMNS - 1) {
                if x == 0 || x >= 6 {
                    assert!(formation.enemies[x][y].is_none());
                } else if ((COLUMNS - ENEMIES_PER_ROW - 1)..(COLUMNS - 1)).contains(&y) {
                    assert!(formation.enemies[x][y].is_some());
                } else {
                    assert!(formation.enemies[x][y].is_none());
                }
            }
        }
    }

    #[test]
    fn should_advance_enemies_to_the_next_row_when_there_is_no_left_space() {
        let mut formation = EnemyFormation::new();

        let row_jumps = 2;
        let fully_traversed_rows = 1;
        let steps_per_row = 30;

        let steps_to_reach_the_beginning_of_the_third_line =
            (fully_traversed_rows * steps_per_row) + row_jumps + FIRST_ENEMY_Y;

        for _ in 0..steps_to_reach_the_beginning_of_the_third_line {
            formation.advance_enemies();
        }

        assert_eq!(formation.status, FormationStatus::Advancing);
        assert_eq!(formation.direction, MovingDirection::ToRight);

        for x in 0..(ROWS - 1) {
            for y in 0..(COLUMNS - 1) {
                if x <= 1 || x >= 7 {
                    assert!(formation.enemies[x][y].is_none());
                } else if y < 11 {
                    assert!(formation.enemies[x][y].is_some());
                } else {
                    assert!(formation.enemies[x][y].is_none());
                }
            }
        }
    }

    #[test]
    fn should_not_advance_anymore_when_end_is_reached() {
        let mut formation = EnemyFormation::new();

        let row_jumps = 10;
        let fully_traversed_rows = 10;
        let steps_per_row = 30;

        let steps_to_reach_the_end =
            (fully_traversed_rows * steps_per_row) + row_jumps + FIRST_ENEMY_Y;

        for _ in 0..steps_to_reach_the_end {
            formation.advance_enemies();
        }

        assert_eq!(formation.status, FormationStatus::Breached);
        formation.advance_enemies();
        assert_eq!(formation.status, FormationStatus::Breached);
    }
}
