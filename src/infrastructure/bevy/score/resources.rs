use crate::domain::score::Score;
use bevy::color::Color;
use bevy::prelude::{Resource, Val};

pub const SCORE_CONTAINER_WIDTH: Val = Val::Percent(50.0);
pub const SCORE_CONTAINER_HEIGHT: Val = Val::Px(50.0);
pub const SCORE_LABEL_HEIGHT: Val = Val::Percent(50.0);
pub const SCORE_LABEL_MARGIN_RIGHT: Val = Val::Px(20.0);
pub const SCORE_LABEL_TEXT: &str = "Score: ";
pub const SCORE_LABEL_FONT_SIZE: f32 = 14.0;
pub const SCORE_LABEL_FONT_COLOR: Color = Color::WHITE;
pub const SCORE_VALUE_HEIGHT: Val = Val::Percent(50.0);
pub const SCORE_VALUE_FONT_SIZE: f32 = 14.0;
pub const SCORE_VALUE_FONT_COLOR: Color = Color::srgb_u8(51, 255, 3);
#[derive(Resource)]
pub struct ScoreResource(pub Score);
