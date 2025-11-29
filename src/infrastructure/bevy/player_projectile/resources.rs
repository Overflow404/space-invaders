use bevy::color::Color;
use bevy::prelude::{Resource, Timer};

pub const PLAYER_PROJECTILE_SPEED: f32 = 500.0;
pub const PLAYER_PROJECTILE_DURATION: f32 = 1.2;
pub const PLAYER_PROJECTILE_WIDTH: f32 = 5.0;
pub const PLAYER_PROJECTILE_HEIGHT: f32 = 15.0;
pub const PLAYER_PROJECTILE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Resource)]
pub struct PlayerProjectileMovementTimerResource(pub Timer);
