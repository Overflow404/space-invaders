use crate::domain::player::Player;
use crate::infrastructure::bevy::game_area::resources::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::prelude::Resource;

pub const PLAYER_IMAGE: &str = "player-green.png";
pub const PLAYER_X: f32 = 0.0;
pub const PLAYER_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.78;
pub const PLAYER_WIDTH: f32 = GAME_AREA_WIDTH * 0.045;
pub const PLAYER_HEIGHT: f32 = GAME_AREA_HEIGHT * 0.043;
pub const DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE: f32 = 25.0;
pub const PLAYER_SPEED: f32 = 300.0;

#[derive(Resource)]
pub struct PlayerResource(pub Player);
