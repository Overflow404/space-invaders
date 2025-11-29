use crate::domain::enemy_formation::EnemyFormation;
use bevy::prelude::{Resource, Timer};

pub const ENEMY_FORMATION_STEP_DURATION: f32 = 0.6;
pub const SPACE_BETWEEN_ENEMIES_X: f32 = 15.0;
pub const SPACE_BETWEEN_ENEMIES_Y: f32 = 15.0;
pub const VERTICAL_DROP: f32 = 15.0;
pub const ENEMY_FIRE_PROBABILITY: f64 = 0.2;

#[derive(Resource)]
pub struct EnemyFormationResource(pub EnemyFormation);

#[derive(Resource)]
pub struct EnemyFormationMovementTimer(pub Timer);
