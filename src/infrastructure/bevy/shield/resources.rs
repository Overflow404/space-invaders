use crate::infrastructure::bevy::game_area::resources::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};

pub const SHIELD_IMAGE: &str = "shield.png";
pub const SHIELD_WIDTH: f32 = GAME_AREA_WIDTH * 0.09;
pub const SHIELD_HEIGHT: f32 = GAME_AREA_HEIGHT * 0.11;
pub const SHIELD_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.58;
pub const SHIELD_X: f32 = -(GAME_AREA_WIDTH / 2.0) * 0.68;
