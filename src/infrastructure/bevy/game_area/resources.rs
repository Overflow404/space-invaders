use crate::infrastructure::bevy::bevy_renderer::resources::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;

pub const GAME_AREA_WIDTH: f32 = WINDOW_WIDTH * 0.93;
pub const GAME_AREA_HEIGHT: f32 = (WINDOW_HEIGHT - HEADER_HEIGHT) * 0.98;
pub const BACKGROUND_IMAGE: &str = "tv.png";
