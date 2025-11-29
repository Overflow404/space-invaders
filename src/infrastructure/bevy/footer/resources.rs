use crate::infrastructure::bevy::game_area::resources::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::color::Color;

pub const LINE_THICKNESS: f32 = 5.0;
pub const LINE_LENGTH: f32 = GAME_AREA_WIDTH;
pub const LINE_X: f32 = 0.0;
pub const LINE_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.90;
pub const LINE_COLOR: Color = Color::srgb(0.2039, 1.0, 0.0);
