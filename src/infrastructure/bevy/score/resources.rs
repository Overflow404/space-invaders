use crate::domain::score::Score;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ScoreResource(pub Score);
