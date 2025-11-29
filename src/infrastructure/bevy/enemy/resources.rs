use bevy::color::Color;
use bevy::prelude::{Resource, Timer};

pub const ENEMY_WIDTH: f32 = 60.0;
pub const ENEMY_HEIGHT: f32 = 40.0;
pub const ENEMY_IMAGE: &str = "red.png";
pub const ENEMY_COLOR: Color = Color::srgb(255.0, 255.0, 255.0);

#[derive(Resource)]
pub struct EnemyProjectileMovementTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyFireProbability(pub f64);
