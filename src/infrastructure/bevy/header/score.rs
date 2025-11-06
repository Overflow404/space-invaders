use bevy::ecs::component::Component;

use crate::domain::score::Score;

#[derive(Component)]
pub struct ScoreComponent(pub Score);

#[derive(Component)]
pub struct ScoreView;
